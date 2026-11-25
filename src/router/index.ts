import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '@/views/HomeView.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: HomeView,
  },
  {
    path: '/discover',
    name: 'Discover',
    component: () => import('@/views/DiscoverView.vue'),
  },
  {
    path: '/library',
    name: 'Library',
    component: () => import('@/views/LibraryView.vue'),
  },
  {
    path: '/playlist/:id',
    name: 'Playlist',
    component: () => import('@/views/PlaylistView.vue'),
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('@/views/SettingsView.vue'),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

