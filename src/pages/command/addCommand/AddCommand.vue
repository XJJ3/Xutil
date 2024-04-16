<template>
  <div class="add_command_page">
    <n-form ref="formRef" :label-width="80" :model="formValue" :rules="rules">
      <n-grid :cols="24" :x-gap="12">
        <n-form-item-gi :span="24" label="命令名称" path="cmdName">
          <n-input v-model:value="formValue.cmdName" placeholder="输入命令名称" maxlength="5" />
        </n-form-item-gi>
        <n-form-item-gi :span="12" label="命令Cmd" path="cmd">
          <n-input v-model:value="formValue.cmd" placeholder="输入命令Cmd" />
        </n-form-item-gi>
        <n-form-item-gi :span="12" label="命令参数">
          <n-dynamic-tags
            v-model:value="formValue.args"
            :input-style="{ maxWidth: '150px' }"
            :tag-style="{ maxWidth: '150px' }"
          />
        </n-form-item-gi>
        <n-form-item-gi :span="12" label="命令图标">
          <n-button attr-type="button" style="width: 180px" @click="handleSelectFile">
            上传文件
          </n-button>
          <n-image
            width="30"
            :src="selectImgBlobUrl"
            object-fit="cover"
            style="margin-left: 10px"
          />
        </n-form-item-gi>
        <n-form-item-gi :span="12" label="执行路径">
          <n-input v-model:value="formValue.currDir" placeholder="输入执行指令需设定的当前路径" />
        </n-form-item-gi>
      </n-grid>

      <n-button attr-type="button" @click="handleValidateClick" style="width: 100%">
        确认添加
      </n-button>
    </n-form>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { readBinaryFile } from '@tauri-apps/api/fs';
import { WebviewWindow, getCurrent } from '@tauri-apps/api/window';
import { FormInst, useMessage } from 'naive-ui';
import { useRoute } from 'vue-router';

const selectImgBlobUrl = ref();
const selectImgUrl = ref();
const message = useMessage();
const route = useRoute();
const formRef = ref<FormInst | null>(null);
const formValue = ref({
  cmdName: '',
  cmd: '',
  icon: '',
  args: [],
  currDir: '',
});
const rules = {
  cmdName: {
    required: true,
    message: '请输入命令名称',
    trigger: ['input'],
  },
  cmd: {
    required: true,
    message: '请输入命令',
    trigger: ['input'],
  },
};

const handleValidateClick = (e: MouseEvent) => {
  e.preventDefault();
  formRef.value?.validate((errors) => {
    if (!errors) addCommandReq();
  });
};

const addCommandReq = () => {
  invoke('dispatch_command', {
    name: 'add_command',
    args: {
      cmd: formValue.value.cmd,
      cmd_name: formValue.value.cmdName,
      args: formValue.value.args,
      current_dir: formValue.value.currDir,
      group_name: route.query.groupName,
      cmd_icon: selectImgUrl.value,
    },
  }).then((response: any) => {
    message.success('添加成功');

    window.setTimeout(() => {
      const win = getCurrent();
      win.close();
      const mainWin = WebviewWindow.getByLabel('main');
      mainWin?.setFocus();
    }, 1500);
  });
};

const handleSelectFile = async () => {
  const filePath = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: 'Image',
        extensions: ['png', 'jpeg', 'svg'],
      },
    ],
  });

  if (!filePath) return;
  const path = Array.isArray(filePath) ? filePath[0] : filePath;
  const contents = await readBinaryFile(path);

  const fileExtension = getFileExtension(path);
  const blob = new Blob([contents], {
    type: `image/${fileExtension === 'svg' ? 'svg+xml' : fileExtension}`,
  });
  const url = URL.createObjectURL(blob);
  selectImgBlobUrl.value = url;
  selectImgUrl.value = path;
};

function getFileExtension(fileName: string) {
  const lastDotIndex = fileName.lastIndexOf('.');
  if (lastDotIndex === -1 || lastDotIndex === fileName.length - 1) return null;
  return fileName.substring(lastDotIndex + 1);
}

onMounted(() => {
  console.log(route.query);
  if (!route.query.groupName) message.warning('未找到分组信息');
});
</script>
<style scoped lang="scss">
.add_command_page {
  width: 100vw;
  height: 100vh;
  background-color: rgb(40, 40, 40);
  position: fixed;
  top: 0;
  left: 0;
  padding: 20px;
  box-sizing: border-box;
}

::v-deep(.n-tag__content) {
  max-width: 85%;
  overflow: hidden; //超出的文本隐藏
  text-overflow: ellipsis; //溢出用省略号显示
  white-space: nowrap; //溢出不换行
}
</style>
