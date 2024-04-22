import type { RouteRecordRaw } from 'vue-router';
import BasicLayout from '@/layout/BasicLayout.vue';

export const routes: RouteRecordRaw[] = [
  {
    path: '/home',
    meta: { title: '首页' },
    redirect: '/home/translate',
    component: BasicLayout,
    children: [
      {
        path: 'command',
        meta: { title: '指令' },
        component: () => import('@/pages/command'),
      },
      {
        path: 'translate',
        meta: { title: '翻译' },
        component: () => import('@/pages/translate'),
      },
      {
        path: 'notice',
        meta: { title: '强提醒' },
        component: () => import('@/pages/notice'),
      },
    ],
  },
  {
    path: '/add-command',
    meta: { title: '添加命令' },
    component: () => import('@/pages/command/addCommand'),
  },
  {
    path: '/add-command-group',
    meta: { title: '添加分组' },
    component: () => import('@/pages/command/addGroup'),
  },
  {
    path: '/add-notice',
    meta: { title: '添加强提醒' },
    component: () => import('@/pages/notice/addNotice'),
  },
  {
    path: '/show-notice',
    meta: { title: '强提醒' },
    component: () => import('@/pages/notice/show'),
  },
];
