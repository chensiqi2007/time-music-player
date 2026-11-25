use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Emitter;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const PASSPORT_BASE_URL: &str = "https://login.urltime.cn";
const PASSPORT_CLIENT_ID: &str = "app-xpnsfgs9";
const USER_AGENT_STR: &str = "TimeMusic/0.1.0 (Tauri; Windows/Linux)";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportUserProfile {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportAuthResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user: PassportUserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportLoginStart {
    pub auth_url: String,
    pub redirect_uri: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportCallbackPayload {
    pub code: Option<String>,
    pub state: Option<String>,
    pub redirect_uri: String,
    pub error: Option<String>,
}

pub struct PassportClient {
    client: Client,
    client_secret: Mutex<Option<String>>,
    pending_state: Mutex<Option<String>>,
    pending_redirect_uri: Mutex<Option<String>>,
}

impl PassportClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(8))
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap_or_else(|_| Client::new());

        let env_secret = std::env::var("URLTIME_SSO_CLIENT_SECRET")
            .ok()
            .filter(|s| !s.trim().is_empty());

        Self {
            client,
            client_secret: Mutex::new(env_secret),
            pending_state: Mutex::new(None),
            pending_redirect_uri: Mutex::new(None),
        }
    }

    pub fn set_client_secret(&self, secret: Option<String>) {
        if let Ok(mut guard) = self.client_secret.lock() {
            *guard = secret.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        }
    }

    fn get_client_secret(&self, override_secret: Option<String>) -> Option<String> {
        if let Some(s) = override_secret.map(|v| v.trim().to_string()).filter(|v| !v.is_empty()) {
            return Some(s);
        }
        self.client_secret.lock().ok().and_then(|g| g.clone())
    }

    /// Generates a 43-char URL-safe random state matching `^[A-Za-z0-9_-]{32,128}$`.
    fn generate_state() -> String {
        let mut bytes = [0u8; 32];
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        for (i, chunk) in bytes.chunks_mut(8).enumerate() {
            let mut hasher = RandomState::new().build_hasher();
            hasher.write_u128(now.wrapping_add(i as u128));
            hasher.write_usize(i);
            let val = hasher.finish().to_le_bytes();
            chunk.copy_from_slice(&val[..chunk.len()]);
        }
        URL_SAFE_NO_PAD.encode(bytes)
    }

    fn build_headers(&self, token: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(t) = token {
            if let Ok(val) = HeaderValue::from_str(&format!("Bearer {}", t)) {
                headers.insert(AUTHORIZATION, val);
            }
        }
        headers
    }

    /// Starts a local loopback HTTP callback server on 127.0.0.1 and returns the SSO authorize URL.
    pub async fn start_login(&self, app: tauri::AppHandle) -> Result<PassportLoginStart, String> {
        let mut listener_opt = None;
        let mut bound_port = 19836u16;
        for port in 19836u16..=19840u16 {
            if let Ok(l) = TcpListener::bind(("127.0.0.1", port)).await {
                bound_port = port;
                listener_opt = Some(l);
                break;
            }
        }

        let listener = listener_opt.ok_or_else(|| {
            "无法在本地启动 SSO 回调监听端口 (19836-19840 均被占用)".to_string()
        })?;

        let redirect_uri = format!("http://127.0.0.1:{}/api/auth/callback", bound_port);
        let state = Self::generate_state();

        if let Ok(mut guard) = self.pending_state.lock() {
            *guard = Some(state.clone());
        }
        if let Ok(mut guard) = self.pending_redirect_uri.lock() {
            *guard = Some(redirect_uri.clone());
        }

        let auth_url = format!(
            "{}/auth/login?client_id={}&redirect_uri={}&state={}",
            PASSPORT_BASE_URL,
            urlencoding::encode(PASSPORT_CLIENT_ID),
            urlencoding::encode(&redirect_uri),
            urlencoding::encode(&state)
        );

        let expected_state = state.clone();
        let redirect_uri_clone = redirect_uri.clone();

        tokio::spawn(async move {
            let timeout = tokio::time::sleep(std::time::Duration::from_secs(300));
            tokio::pin!(timeout);

            loop {
                tokio::select! {
                    _ = &mut timeout => {
                        break;
                    }
                    accept_res = listener.accept() => {
                        let Ok((mut socket, _)) = accept_res else { break; };
                        let mut buf = [0u8; 4096];
                        let n = match socket.read(&mut buf).await {
                            Ok(n) if n > 0 => n,
                            _ => continue,
                        };
                        let req_str = String::from_utf8_lossy(&buf[..n]);
                        let first_line = req_str.lines().next().unwrap_or("");
                        let path_and_query = first_line.split_whitespace().nth(1).unwrap_or("/");

                        // Ignore favicon or unrelated browser preflight requests
                        if !path_and_query.starts_with("/api/auth/callback") {
                            let not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                            let _ = socket.write_all(not_found.as_bytes()).await;
                            continue;
                        }

                        let query_str = path_and_query.splitn(2, '?').nth(1).unwrap_or("");
                        let mut code: Option<String> = None;
                        let mut state_param: Option<String> = None;
                        let mut err_param: Option<String> = None;

                        for pair in query_str.split('&') {
                            let mut kv = pair.splitn(2, '=');
                            let k = kv.next().unwrap_or("");
                            let v = kv.next().unwrap_or("");
                            let decoded = urlencoding::decode(v).map(|s| s.into_owned()).unwrap_or_else(|_| v.to_string());
                            match k {
                                "code" => code = Some(decoded),
                                "state" => state_param = Some(decoded),
                                "error" | "error_description" => err_param = Some(decoded),
                                _ => {}
                            }
                        }

                        let state_valid = state_param.as_deref() == Some(expected_state.as_str());
                        let (title, subtitle, final_err) = if let Some(e) = err_param {
                            ("授权未完成", format!("认证中心返回错误：{}", e), Some(e))
                        } else if !state_valid {
                            ("安全校验失败", "State 参数不匹配，请回到 Time Music 重试登录。".to_string(), Some("State 校验失败".to_string()))
                        } else if code.as_ref().map(|c| !c.is_empty()).unwrap_or(false) {
                            ("登录成功", "已完成 URL Time Passport 授权，您可以关闭此页面并返回 Time Music。".to_string(), None)
                        } else {
                            ("缺少授权码", "回调地址中未包含有效的 code 参数。".to_string(), Some("未收到授权码".to_string()))
                        };

                        let html = format!(
                            "<!DOCTYPE html><html lang=\"zh-CN\"><head><meta charset=\"utf-8\"><title>{title} · Time Music</title>\
                            <style>body{{margin:0;min-height:100vh;display:flex;align-items:center;justify-content:center;background:#0e0f14;color:#f8fafc;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;}}\
                            .card{{background:rgba(255,255,255,0.05);border:1px solid rgba(255,255,255,0.12);border-radius:20px;padding:32px 36px;max-width:420px;text-align:center;box-shadow:0 20px 50px rgba(0,0,0,0.45);}}\
                            h1{{margin:0 0 10px;font-size:20px;}}p{{margin:0;color:#94a3b8;font-size:14px;line-height:1.6;}}</style></head>\
                            <body><div class=\"card\"><h1>{title}</h1><p>{subtitle}</p></div></body></html>"
                        );
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            html.as_bytes().len(),
                            html
                        );
                        let _ = socket.write_all(response.as_bytes()).await;

                        let _ = app.emit(
                            "passport:callback",
                            PassportCallbackPayload {
                                code: if final_err.is_none() { code } else { None },
                                state: state_param,
                                redirect_uri: redirect_uri_clone,
                                error: final_err,
                            },
                        );
                        break;
                    }
                }
            }
        });

        Ok(PassportLoginStart {
            auth_url,
            redirect_uri,
            state,
        })
    }

    pub async fn exchange_code(
        &self,
        code: &str,
        redirect_uri: Option<String>,
        client_secret: Option<String>,
    ) -> Result<PassportAuthResult, String> {
        let resolved_redirect = redirect_uri
            .filter(|s| !s.trim().is_empty())
            .or_else(|| self.pending_redirect_uri.lock().ok().and_then(|g| g.clone()))
            .unwrap_or_else(|| "http://127.0.0.1:19836/api/auth/callback".to_string());

        let mut payload = serde_json::json!({
            "code": code.trim(),
            "client_id": PASSPORT_CLIENT_ID,
            "redirect_uri": resolved_redirect,
        });

        if let Some(secret) = self.get_client_secret(client_secret) {
            payload["client_secret"] = serde_json::Value::String(secret);
        }

        let url = format!("{}/api/v1/sso/token", PASSPORT_BASE_URL);
        let res = self
            .client
            .post(&url)
            .headers(self.build_headers(None))
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("连接 URL Time Passport 失败: {}", e))?;

        let status = res.status();
        let json: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("解析 Passport 响应失败: {}", e))?;

        if !status.is_success() || json.get("code").and_then(|c| c.as_i64()).unwrap_or(200) >= 400 {
            let msg = json
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("SSO 凭证交换失败");
            return Err(msg.to_string());
        }

        let data = json.get("data").unwrap_or(&json);
        let access_token = data
            .get("access_token")
            .or_else(|| data.get("token"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "Passport 未返回 access_token".to_string())?;

        let refresh_token = data
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let user = if let Some(u) = data.get("user") {
            Self::parse_user_value(u)
        } else {
            self.get_user_info(&access_token).await?
        };

        Ok(PassportAuthResult {
            access_token,
            refresh_token,
            user,
        })
    }

    pub async fn get_user_info(&self, token: &str) -> Result<PassportUserProfile, String> {
        let url = format!("{}/api/v1/users/me", PASSPORT_BASE_URL);
        let res = self
            .client
            .get(&url)
            .headers(self.build_headers(Some(token)))
            .send()
            .await
            .map_err(|e| format!("获取用户信息失败: {}", e))?;

        let status = res.status();
        let json: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("解析用户信息失败: {}", e))?;

        if !status.is_success() {
            let msg = json
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("登录会话已过期");
            return Err(msg.to_string());
        }

        let data = json.get("data").unwrap_or(&json);
        let user_obj = data.get("user").unwrap_or(data);
        Ok(Self::parse_user_value(user_obj))
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<PassportAuthResult, String> {
        let url = format!("{}/api/v1/auth/refresh", PASSPORT_BASE_URL);
        let res = self
            .client
            .post(&url)
            .headers(self.build_headers(None))
            .json(&serde_json::json!({ "refresh_token": refresh_token }))
            .send()
            .await
            .map_err(|e| format!("刷新令牌失败: {}", e))?;

        let status = res.status();
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        if !status.is_success() {
            return Err("刷新登录会话失败，请重新登录".to_string());
        }

        let data = json.get("data").unwrap_or(&json);
        let access_token = data
            .get("access_token")
            .or_else(|| data.get("token"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "未返回新令牌".to_string())?;

        let new_refresh = data
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| Some(refresh_token.to_string()));

        let user = self.get_user_info(&access_token).await?;
        Ok(PassportAuthResult {
            access_token,
            refresh_token: new_refresh,
            user,
        })
    }

    pub async fn logout(&self, token: Option<String>) {
        if let Some(t) = token {
            let url = format!("{}/api/v1/auth/logout", PASSPORT_BASE_URL);
            let _ = self
                .client
                .post(&url)
                .headers(self.build_headers(Some(&t)))
                .send()
                .await;
        }
    }

    fn parse_user_value(v: &serde_json::Value) -> PassportUserProfile {
        let id = v
            .get("id")
            .or_else(|| v.get("user_id"))
            .map(|val| {
                if let Some(s) = val.as_str() {
                    s.to_string()
                } else if let Some(n) = val.as_i64() {
                    n.to_string()
                } else {
                    "0".to_string()
                }
            })
            .unwrap_or_else(|| "0".to_string());

        let username = v
            .get("username")
            .or_else(|| v.get("name"))
            .and_then(|s| s.as_str())
            .unwrap_or("time_user")
            .to_string();

        let nickname = v
            .get("nickname")
            .or_else(|| v.get("display_name"))
            .or_else(|| v.get("username"))
            .or_else(|| v.get("name"))
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(&username)
            .to_string();

        let email = v
            .get("email")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        let avatar_url = v
            .get("avatar_url")
            .or_else(|| v.get("avatar"))
            .and_then(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        let role = v
            .get("role")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        PassportUserProfile {
            id,
            username,
            nickname,
            email,
            avatar_url,
            role,
        }
    }
}
