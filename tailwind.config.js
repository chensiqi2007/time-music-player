/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        theme: {
          base: 'var(--bg-base)',
          titlebar: 'var(--bg-titlebar)',
          surface: 'var(--bg-surface)',
          surfaceElevated: 'var(--bg-surface-elevated)',
          card: 'var(--bg-card)',
          cardHover: 'var(--bg-card-hover)',
          sidebar: 'var(--bg-sidebar)',
          player: 'var(--bg-player)',
          border: 'var(--border-color)',
          subtle: 'var(--border-subtle)',
          sliderTrack: 'var(--slider-track)',
          text: 'var(--text-primary)',
          muted: 'var(--text-secondary)',
        },
        brand: {
          DEFAULT: 'var(--brand-color)',
          light: 'var(--brand-light)',
          glow: 'var(--brand-glow)',
        },
      },
      borderRadius: {
        '2xl': '1rem',
        '3xl': '1.5rem',
      },
      backdropBlur: {
        '2xl': '40px',
        '3xl': '64px',
      },
      animation: {
        'pulse-subtle': 'pulseSubtle 3s ease-in-out infinite',
        'float-slow': 'floatSlow 14s ease-in-out infinite alternate',
        'float-delayed': 'floatDelayed 18s ease-in-out infinite alternate',
        'bar-1': 'barDance 1s ease-in-out infinite alternate',
        'bar-2': 'barDance 1.3s ease-in-out 0.2s infinite alternate',
        'bar-3': 'barDance 0.9s ease-in-out 0.4s infinite alternate',
        'bar-4': 'barDance 1.1s ease-in-out 0.1s infinite alternate',
      },
      keyframes: {
        pulseSubtle: {
          '0%, 100%': { opacity: 0.8, transform: 'scale(1)' },
          '50%': { opacity: 1, transform: 'scale(1.03)' },
        },
        floatSlow: {
          '0%': { transform: 'translate(0, 0) scale(1)' },
          '50%': { transform: 'translate(40px, -30px) scale(1.1)' },
          '100%': { transform: 'translate(-20px, 30px) scale(0.95)' },
        },
        floatDelayed: {
          '0%': { transform: 'translate(0, 0) scale(1)' },
          '50%': { transform: 'translate(-50px, 40px) scale(1.15)' },
          '100%': { transform: 'translate(30px, -20px) scale(0.9)' },
        },
        barDance: {
          '0%': { height: '20%' },
          '100%': { height: '100%' },
        },
      },
    },
  },
  plugins: [],
}
