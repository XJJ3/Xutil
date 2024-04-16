<template>
  <div class="add_command_page">
    <n-config-provider :theme="darkTheme">
      <n-form ref="formRef" :label-width="80" :model="formValue" :rules="rules">
        <n-grid :cols="24" :x-gap="12">
          <n-form-item-gi :span="12" label="分组名称" path="groupName">
            <n-input v-model:value="formValue.groupName" placeholder="输入分组名称" />
          </n-form-item-gi>
          <n-form-item-gi :span="12" label="图标" path="groupIcon">
            <n-button attr-type="button" style="width: 180px" @click="handleSelectFile">
              上传文件
            </n-button>
            <n-image width="30" :src="selectImgUrl" object-fit="cover" style="margin-left: 10px" />
          </n-form-item-gi>
        </n-grid>

        <n-button attr-type="button" @click="handleValidateClick" style="width: 100%">
          确认添加
        </n-button>
      </n-form>
    </n-config-provider>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { readBinaryFile } from '@tauri-apps/api/fs';
import { WebviewWindow, getCurrent } from '@tauri-apps/api/window';
import { FormInst, useMessage } from 'naive-ui';
import { darkTheme } from 'naive-ui';

const selectImgUrl = ref();
const message = useMessage();
const formRef = ref<FormInst | null>(null);
const formValue = ref({
  groupName: '',
  groupIcon: '',
});
const rules = {
  groupName: {
    required: true,
    message: '请输入名称',
    trigger: ['input'],
  },
  groupIcon: {
    required: true,
    message: '请选择图标',
    trigger: ['change'],
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
    name: 'add_command_group',
    args: {
      group_name: formValue.value.groupName,
      group_icon: formValue.value.groupIcon,
    },
  }).then((response) => {
    console.log(response);
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
  selectImgUrl.value = url;
  formValue.value.groupIcon = path;
};

function getFileExtension(fileName: string) {
  const lastDotIndex = fileName.lastIndexOf('.');
  if (lastDotIndex === -1 || lastDotIndex === fileName.length - 1) return null;
  return fileName.substring(lastDotIndex + 1);
}
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
</style>
