<template>
  <div class="notice_wrapper">
    <simplebar class="notice_list" data-tauri-drag-region>
      <div class="notice_item">
        <n-checkbox style="margin-right: 10px"></n-checkbox>
        <span>项目名称</span>
        <span>提醒方式</span>
      </div>
      <div class="notice_item">
        <n-checkbox style="margin-right: 10px"></n-checkbox>
        <span>项目名称</span>
        <span>提醒方式</span>
      </div>
      <n-button dashed style="width: 60%; margin: 20px 20% 16px" @click="handleNewNotice">
        <template #icon>
          <svg class="icon" viewBox="0 0 1024 1024" width="48" height="48" fill="currentColor">
            <path
              d="M469.333334 213.333334 469.333334 469.333334 213.333334 469.333334 213.333334 554.666666 469.333334 554.666666 469.333334 810.666664 554.666666 810.666664 554.666666 554.666666 810.666664 554.666666 810.666664 469.333334 554.666666 469.333334 554.666666 213.333334 469.333334 213.333334Z"
              p-id="3024"
            ></path>
          </svg>
        </template>
        添加
      </n-button>
    </simplebar>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/window';
import simplebar from 'simplebar-vue';
import 'simplebar-vue/dist/simplebar.min.css';

const getAllNotice = () => {
  invoke('dispatch_command', {
    name: 'get_scheduler_job_list',
    args: {},
  }).then((response: any) => {
    console.log(response);
  });
};

const handleNewNotice = () => {
  const webview = WebviewWindow.getByLabel('addNotice');
  if (!webview) {
    new WebviewWindow('addNotice', {
      url: `#/add-notice`,
      fullscreen: false,
      height: 480,
      resizable: false,
      title: '添加强提醒',
      width: 650,
      alwaysOnTop: true,
      transparent: true,
    });
  } else {
    webview.setFocus();
  }

  // const webview = WebviewWindow.getByLabel('showNotice');
  // if (!webview) {
  //   new WebviewWindow('showNotice', {
  //     url: `#/show-notice`,
  //     fullscreen: false,
  //     width: window.screen.width,
  //     height: window.screen.height,
  //     resizable: false,
  //     title: '添加强提醒',
  //     alwaysOnTop: true,
  //     decorations: false,
  //     transparent: true,
  //   });
  // } else {
  //   webview.setFocus();
  // }
};

onMounted(async () => {
  getAllNotice();

  const listWrapper = document.getElementsByClassName('simplebar-content-wrapper')[0];
  if (listWrapper) listWrapper.setAttribute('data-tauri-drag-region', 'true');

  const listContent = document.getElementsByClassName('simplebar-content')[0];
  if (listContent) listContent.setAttribute('data-tauri-drag-region', 'true');
});
</script>
<style lang="scss" scoped>
.notice_wrapper {
  width: 100%;
  height: 100%;
  // background: rgba(0, 0, 0, 0.3);

  .notice_list {
    width: 100%;
    height: 100%;
    padding: 12px 0;
    box-sizing: border-box;

    .notice_item {
      // background: linear-gradient(to left, rgb(189, 195, 199), rgb(44, 62, 80));
      background: linear-gradient(to right, rgb(15, 32, 39), rgb(32, 58, 67), rgb(44, 83, 100));
      border-radius: 4px;
      padding: 0 2px;
      box-sizing: border-box;
      display: flex;
      align-items: center;
      margin-bottom: 10px;
    }
  }
}
</style>
