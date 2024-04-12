import { RouteRecordRaw, createRouter, createWebHashHistory } from 'vue-router';

import { routes } from './routes';

const basicRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/home',
    meta: {
      title: '首页',
    },
  },
  {
    path: '/:catchAll(.*)*',
    name: 'NotFound',
    redirect: '/',
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes: [...basicRoutes, ...routes],
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition;
    }
    return { top: 0 };
  },
});

router.beforeEach((to, from, next) => {
  document.title = (to.meta.title as string) || import.meta.env.VITE_APP_TITLE;
  next();
});

export default router;
