<template>
  <div class="command_wrapper" data-tauri-drag-region>
    <div class="command_content" data-tauri-drag-region>
      <div class="command_list" data-tauri-drag-region>
        <div v-for="(item, index) in selectedGroupCmdArr" :key="index" class="btn middle">
          <img :src="item.cmdIcon" />
          <span>{{ item.cmd_name }}</span>

          <div :class="['item_operation', item.loading && 'stop']">
            <div class="opera execute" @click="() => handleRunCmd(index)">
              <svg class="opera_icon" viewBox="0 0 1024 1024" version="1.1" width="48" height="48">
                <path
                  d="M512 128a382.6 382.6 0 1 1-149.45 30.15A381.54 381.54 0 0 1 512 128m0-64C264.58 64 64 264.58 64 512s200.58 448 448 448 448-200.58 448-448S759.42 64 512 64z"
                ></path>
                <path d="M352 256v512l448-256-448-256z"></path>
              </svg>
              <span>执行</span>
            </div>

            <div class="opera del" @click="() => handleDelCmd(index)">
              <svg class="opera_icon" viewBox="0 0 1024 1024" version="1.1" width="48" height="48">
                <path
                  d="M519.620465 0c-103.924093 0-188.511256 82.467721-192.083349 185.820279H85.015814A48.91386 48.91386 0 0 0 36.101953 234.686512a48.91386 48.91386 0 0 0 48.913861 48.866232h54.010046V831.345116c0 102.852465 69.822512 186.844279 155.909954 186.844279h439.200744c86.087442 0 155.909953-83.491721 155.909954-186.844279V284.100465h48.91386a48.91386 48.91386 0 0 0 48.913861-48.890046 48.91386 48.91386 0 0 0-48.913861-48.866233h-227.756651A191.559442 191.559442 0 0 0 519.620465 0z m-107.234232 177.080558c3.548279-49.771163 46.627721-88.540279 99.851907-88.540279 53.224186 0 96.327442 38.745302 99.351813 88.540279h-199.20372z m-111.997024 752.044651c-30.981953 0-65.083535-39.15014-65.083535-95.041488V287.744h575.488v546.839814c0 55.915163-34.077767 95.041488-65.059721 95.041488H300.389209v-0.500093z"
                ></path>
                <path
                  d="M368.116093 796.814884c24.361674 0 44.27014-21.670698 44.27014-48.818605v-278.623256c0-27.147907-19.908465-48.818605-44.27014-48.818604-24.33786 0-44.27014 21.670698-44.27014 48.818604v278.623256c0 27.147907 19.360744 48.818605 44.293954 48.818605z m154.933581 0c24.361674 0 44.293953-21.670698 44.293954-48.818605v-278.623256c0-27.147907-19.932279-48.818605-44.293954-48.818604-24.33786 0-44.27014 21.670698-44.270139 48.818604v278.623256c0 27.147907 19.932279 48.818605 44.293953 48.818605z m132.810419 0c24.33786 0 44.27014-21.670698 44.27014-48.818605v-278.623256c0-27.147907-19.932279-48.818605-44.27014-48.818604s-44.27014 21.670698-44.27014 48.818604v278.623256c0 27.147907 19.360744 48.818605 44.27014 48.818605z"
                ></path>
              </svg>
              <span>删除</span>
            </div>
          </div>

          <div :class="['loading_wrapper', item.loading && 'show']">
            <img src="@/assets/command/loading.svg" alt="" />
          </div>
        </div>
      </div>

      <div class="tool">
        <div class="tool_item" @click="handleNewCommand">
          <img src="@/assets/command/add.svg" />
          <span>添加</span>
        </div>

        <div class="tool_item">
          <img src="@/assets/command/del.svg" />
          <span>删除</span>
        </div>
      </div>
    </div>
    <div class="command_menu" data-tauri-drag-region>
      <div
        v-for="(item, index) in groupList"
        :key="index"
        class="btn_command_item"
        @click="() => handleSelectGroup(index)"
      >
        <!-- <n-image class="icon" width="30" :src="item.iconUrl" object-fit="cover" /> -->
        <n-popover trigger="hover" placement="left">
          <template #trigger>
            <img class="icon" :src="item.iconUrl" alt="" />
          </template>
          <span>{{ item.group_name }}</span>
        </n-popover>
      </div>

      <div class="btn_add" @click="handleNewCommandGroup">
        <svg
          t="1713150812171"
          class="icon"
          viewBox="0 0 1024 1024"
          version="1.1"
          p-id="3023"
          width="48"
          height="48"
        >
          <path
            d="M469.333334 213.333334 469.333334 469.333334 213.333334 469.333334 213.333334 554.666666 469.333334 554.666666 469.333334 810.666664 554.666666 810.666664 554.666666 554.666666 810.666664 554.666666 810.666664 469.333334 554.666666 469.333334 554.666666 213.333334 469.333334 213.333334Z"
            p-id="3024"
          ></path>
        </svg>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event';
import { readBinaryFile } from '@tauri-apps/api/fs';
import { WebviewWindow } from '@tauri-apps/api/window';
import { ref } from 'vue';

const message = useMessage();
const groupList = ref<any[]>([]);
const selectedGroupIndex = ref(0);
const selectedGroupCmdArr = ref<any[]>([]);

const getAllCommand = () => {
  invoke('dispatch_command', { name: 'get_all_commands', args: {} }).then(async (response: any) => {
    const res = JSON.parse(response.result);

    if (Array.isArray(res)) {
      groupList.value = res;
      getLocalImage(groupList.value, 'iconUrl');
      handleSelectGroup(selectedGroupIndex.value);
    }
  });
};

const getLocalImage = (obj: Array<any>, key: string) => {
  obj.forEach(async (item) => {
    const path = item.group_icon || item.cmd_icon;
    if (!path) return;
    const contents = await readBinaryFile(path);
    const fileExtension = getFileExtension(path);
    const blob = new Blob([contents], {
      type: `image/${fileExtension === 'svg' ? 'svg+xml' : fileExtension}`,
    });
    const url = URL.createObjectURL(blob);
    item[key] = url;
  });
};

const handleSelectGroup = (index: number) => {
  selectedGroupIndex.value = index;
  if (selectedGroupCmdArr.value.length <= 0)
    selectedGroupCmdArr.value = groupList.value[index].commands;
  else {
    selectedGroupCmdArr.value = groupList.value[index].commands.map((item: any) => {
      const has = selectedGroupCmdArr.value.find((i) => i.cmd_name == item.cmd_name);
      return { ...item, loading: has ? has.loading : false };
    });
  }
  nextTick(() => getLocalImage(selectedGroupCmdArr.value, 'cmdIcon'));
};

const handleRunCmd = (index: number) => {
  const cmd = selectedGroupCmdArr.value[index];
  selectedGroupCmdArr.value[index]['loading'] = true;
  invoke('dispatch_command', {
    name: 'execute_cmd',
    args: {
      cmd: cmd.cmd,
      args: cmd.args,
      current_dir: !!cmd.current_dir ? cmd.current_dir : undefined,
    },
  }).then((response) => {
    console.log(response);
    message.success('执行结束');
    selectedGroupCmdArr.value[index]['loading'] = false;
  });
};

const handleDelCmd = (index: number) => {
  const cmd = selectedGroupCmdArr.value[index];
  invoke('dispatch_command', {
    name: 'del_command',
    args: {
      cmd_name: cmd.cmd_name,
      group_name: groupList.value[selectedGroupIndex.value].group_name,
    },
  }).then((response: any) => {
    getAllCommand();
  });
};

const handleClick4 = () => {
  emit('blur');
};

let unListen: () => void;
onMounted(async () => {
  getAllCommand();
  unListen = await listen('windowFocused', (event) => {
    if (event.payload) getAllCommand();
  });
});

onUnmounted(() => {
  unListen && unListen();
});

const handleNewCommand = () => {
  if (!groupList.value?.[selectedGroupIndex.value]?.group_name) {
    handleNewCommandGroup();
    return;
  }
  const webview = WebviewWindow.getByLabel('addCommand');

  if (!webview) {
    new WebviewWindow('addCommand', {
      url: `http://localhost:5173/#/add-command?groupName=${
        groupList.value[selectedGroupIndex.value].group_name
      }`,
      fullscreen: false,
      height: 350,
      resizable: false,
      title: '添加命令',
      width: 500,
      alwaysOnTop: true,
    });
  } else {
    console.log(webview);
    webview.setFocus();
  }
};

const handleNewCommandGroup = () => {
  const webview = WebviewWindow.getByLabel('addCommandGroup');

  if (!webview) {
    new WebviewWindow('addCommandGroup', {
      url: 'http://localhost:5173/#/add-command-group',
      fullscreen: false,
      height: 180,
      resizable: false,
      title: '添加分组',
      width: 500,
      alwaysOnTop: true,
    });
  } else {
    console.log(webview);
    webview.setFocus();
  }
};

function getFileExtension(fileName: string) {
  const lastDotIndex = fileName.lastIndexOf('.');
  if (lastDotIndex === -1 || lastDotIndex === fileName.length - 1) return null;
  return fileName.substring(lastDotIndex + 1);
}
</script>

<style scoped lang="scss">
.command_wrapper {
  width: 100%;
  height: 100%;
  display: flex;

  .command_content {
    width: calc(100% - 40px);
    height: 100%;
    border: 1px solid rgba(255, 255, 255, 0.2);
    box-sizing: border-box;
    border-radius: 8px;
    background-color: rgba(0, 0, 0, 0.2);

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
        position: relative;
        overflow: hidden;

        font-size: 13px;
        color: white;

        &:nth-child(2n) {
          margin-right: 0;
        }

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

        .item_operation {
          // background-color: rgb(227, 75, 75);
          width: 100%;
          height: 100%;
          position: absolute;
          top: 0;
          left: 100%;
          border-radius: 5px;
          transition: left 0.3s;

          display: flex;
          align-items: center;

          &.stop {
            left: 0 !important;
          }

          .opera {
            width: 50%;
            height: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;

            &.execute {
              background: rgb(43, 150, 50);
            }

            &.del {
              background: rgb(215, 44, 44);
            }

            .opera_icon {
              width: 14px;
              height: 14px;
              fill: white;
            }

            & > span {
              font-size: 12px;
              color: white;
              transform: translate(0, -15%) scale(0.7);
              margin-bottom: -6px;
            }
          }
        }

        &:hover {
          .item_operation {
            left: 0;
          }
        }

        .loading_wrapper {
          width: 100%;
          height: 100%;
          position: absolute;
          top: 100%;
          left: 0;
          border-radius: 5px;
          background-color: rgba(0, 0, 0, 0.5);
          // background-color: red;
          display: flex;
          justify-content: center;
          align-items: center;

          & > img {
            width: 20px;
            height: 20px;

            animation: spin 1s linear infinite;
          }

          &.show {
            top: 0;
          }
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
        border-radius: 28px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;

        opacity: 0.5;

        & > img {
          height: 60%;
        }

        & > span {
          font-size: 12px;
          line-height: 1;
          margin-left: 4px;
        }

        &:first-of-type {
          background: linear-gradient(57deg, #4a7dff, #1950ef);
        }

        &:last-of-type {
          background: linear-gradient(57deg, #c05868, #da3047);
        }

        &:hover {
          opacity: 1;
        }
      }
    }
  }

  .command_menu {
    width: 40px;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2px;
    box-sizing: border-box;

    .btn_add {
      width: 32px;
      height: 32px;
      border: 2px solid gray;
      border-radius: 8px;
      display: flex;
      justify-content: center;
      align-items: center;
      box-sizing: border-box;
      color: gray;
      cursor: pointer;

      & > svg {
        width: 90%;
        height: 90%;
        fill: currentColor;
      }
    }

    .btn_command_item {
      width: 32px;
      height: 32px;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 8px;
      margin-bottom: 8px;
      border: 0;
      cursor: pointer;
      position: relative;

      .icon {
        width: 100%;
        height: 100%;
        border-radius: 8px;

        transition: all 0.3s;

        &:hover {
          transform: scale(1.2);
        }
      }
    }
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
