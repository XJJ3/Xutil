<template>
  <div class="command_wrapper" data-tauri-drag-region>
    <!-- <button @click="handleClick"> 写入 </button>

    <button @click="handleClick2"> 打开 </button>

    <button @click="handleClick3"> 执行 </button>

    <button @click="handleClick4"> 事件 </button> -->

    <!-- <div>{{ result }}</div> -->

    <div class="command_list">
      <!-- <div class="btn middle"></div> -->
      <div class="btn middle">
        <img src="@/assets/command/open.svg" alt="" />
        <span>打开项目</span>
      </div>
    </div>

    <div class="tool">
      <div class="tool_item" @click="handleNewCommand">
        <img src="@/assets/command/add.svg" alt="" />
        <span>添加</span>
      </div>

      <div class="tool_item"></div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { WebviewWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';

const result = ref('');

const handleClick = () => {
  invoke('dispatch_command', {
    name: 'add_command',
    args: { cmd: 'sh', args: ['/xujunjie/aaa/cmd.sh'], current_dir: '/' },
  }).then((response) => console.log(response));
};
const handleClick2 = () => {
  invoke('dispatch_command', { name: 'get_all_commands', args: {} }).then((response) =>
    console.log(response)
  );
};

const handleClick3 = () => {
  invoke('dispatch_command', { name: 'execute_cmd', args: { cmd: 'ls', args: [] } }).then(
    (response) => {
      console.log(response);
      result.value += JSON.stringify(response);
    }
  );
};

const handleClick4 = () => {
  emit('blur');
};

const handleNewCommand = () => {
  const webview = WebviewWindow.getByLabel('addCommand');

  if (!webview) {
    new WebviewWindow('addCommand', {
      url: 'https://tauri.app/',
      fullscreen: false,
      height: 300,
      resizable: false,
      title: '添加命令',
      width: 500,
    });
  } else {
    console.log(webview);
    webview.setFocus();
  }
};
</script>

<style scoped lang="scss">
.command_wrapper {
  width: 100%;
  height: 100%;
  padding: 10px;
  box-sizing: border-box;
  position: relative;

  .command_list {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-start;

    .btn {
      height: 32px;
      background-color: rgba(255, 255, 255, 0.2);
      border-radius: 5px;
      margin-bottom: 12px;
      margin-right: 12px;
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 2px;
      box-sizing: border-box;
      cursor: pointer;

      font-size: 13px;
      color: white;

      &.small {
        width: calc((100% - (12px * 2)) / 3);
      }

      &.middle {
        width: calc((100% - 12px) / 2);
      }

      &.big {
        width: 100%;
        margin-right: 0;
      }

      & > img {
        height: 18px;
        margin-right: 2px;
      }
    }
  }

  .tool {
    width: 90%;
    height: 32px;
    position: absolute;
    bottom: 12px;
    left: 0;
    right: 0;
    margin: auto;
    border-radius: 32px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 2px;
    box-sizing: border-box;
    border: 1px solid rgba(255, 255, 255, 0.2);

    .tool_item {
      width: 48%;
      height: 100%;
      background-color: red;
      border-radius: 28px;
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;

      opacity: 0.6;

      & > img {
        height: 70%;
      }

      & > span {
        font-size: 14px;
        line-height: 1;
        margin-left: 4px;
      }

      &:first-of-type {
        background: linear-gradient(57deg, #4a7dff, #1950ef);
      }

      &:last-of-type {
        background: linear-gradient(57deg, #8466e9, #664ff0);
      }

      &:hover {
        opacity: 1;
      }
    }
  }
}
</style>
