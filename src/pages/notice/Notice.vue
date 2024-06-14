<template>
  <div class="notice_wrapper" data-tauri-drag-region>
    <simplebar class="notice_list" data-tauri-drag-region>
      <div v-for="(item, index) in allSchedulerJob" :key="index" class="notice_item">
        <!-- <n-checkbox style="margin-right: 10px"></n-checkbox> -->
        <n-switch v-model:value="item.is_run" size="small" @change="() => handleSwitch(index)" />
        <div class="notice_title">
          <div class="text">{{ item.notice_title }}</div>
          <!-- <span class="title_position">{{ item.title_position }}</span> -->
        </div>
      </div>
    </simplebar>

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
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/window';
import simplebar from 'simplebar-vue';
import 'simplebar-vue/dist/simplebar.min.css';

const allSchedulerJob = ref<any[]>([]);

const getAllNotice = () => {
  invoke('dispatch_command', {
    name: 'get_scheduler_job_list',
    args: {},
  }).then((response: any) => {
    const res = JSON.parse(response.result);
    if (Array.isArray(res)) {
      console.log(res);
      allSchedulerJob.value = res;
    }
  });
};

const handleSwitch = (index: number) => {
  const scheduler = allSchedulerJob.value[index];
  if (scheduler) {
    invoke('dispatch_command', {
      name: 'switch_scheduler_job',
      args: scheduler,
    });
  }
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
      width: 720,
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

let unListen: () => void;
onMounted(async () => {
  getAllNotice();
  unListen = await listen('windowFocused', (event) => {
    if (event.payload) getAllNotice();
  });

  const listWrapper = document.getElementsByClassName('simplebar-content-wrapper')[0];
  if (listWrapper) listWrapper.setAttribute('data-tauri-drag-region', 'true');

  const listContent = document.getElementsByClassName('simplebar-content')[0];
  if (listContent) listContent.setAttribute('data-tauri-drag-region', 'true');
});

onUnmounted(() => {
  unListen && unListen();
});
</script>
<style lang="scss" scoped>
.notice_wrapper {
  width: 100%;
  height: 100%;
  // background: rgba(0, 0, 0, 0.3);

  .notice_list {
    width: 100%;
    max-height: calc(100% - 60px);
    padding-top: 12px;
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

      .notice_title {
        position: relative;
        margin-left: 10px;
        max-width: 160px;

        .text {
          overflow: hidden; //超出的文本隐藏
          text-overflow: ellipsis; //溢出用省略号显示
          white-space: nowrap; //溢出不换行
        }

        .title_position {
          position: absolute;
          right: 0;
          top: 0;
          background-color: gray;
          border-radius: 4px;
          padding: 0 4px;
          transform: translate(80%, -40%) scale(0.6);
        }
      }
    }
  }
}
</style>
