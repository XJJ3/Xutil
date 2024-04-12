import type { RouteRecordRaw } from 'vue-router';
import BasicLayout from '@/layout/BasicLayout.vue';

export const routes: RouteRecordRaw[] = [
  {
    path: '/home',
    meta: { title: '首页' },
    redirect: '/home/command',
    component: BasicLayout,
    children: [
      {
        path: 'command',
        meta: { title: '指令' },
        component: () => import('@/pages/command'),
      },
    ],
  },
  {
    path: '/add-command',
    meta: { title: '添加指令' },
    component: () => import('@/pages/command/addCommand'),
  },
];
