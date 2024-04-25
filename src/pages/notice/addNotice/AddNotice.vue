<template>
  <div class="add_notice_page">
    <n-form ref="formRef" :label-width="80" :model="formValue" :rules="rules">
      <n-grid :cols="24" :x-gap="12">
        <n-form-item-gi :span="24" label="触发事件" path="scheduler">
          <CronNaive
            v-model:model-value="formValue.scheduler"
            @error="error = $event"
            locale="zh"
          ></CronNaive>
        </n-form-item-gi>
        <n-form-item-gi :span="24" label="提醒文字" path="notice_title">
          <n-input v-model:value="formValue.notice_title" placeholder="输入提醒文字" />
        </n-form-item-gi>
        <n-form-item-gi :span="12" label="文字位置">
          <n-radio-group v-model:value="formValue.title_position" name="radiogroup">
            <n-space>
              <n-radio v-for="nType in noticeType" :key="nType.value" :value="nType.value">
                {{ nType.label }}
              </n-radio>
            </n-space>
          </n-radio-group>
        </n-form-item-gi>

        <n-form-item-gi :span="12" label="字体大小">
          <n-input-number v-model:value="formValue.font_size" clearable :min="12" :max="100" />
        </n-form-item-gi>
        <n-form-item-gi :span="12" label="文字颜色">
          <n-color-picker
            v-model:value="formValue.title_color"
            :show-preview="true"
            :show-alpha="false"
          />
        </n-form-item-gi>
        <n-form-item-gi :span="12" label="提示背景">
          <n-color-picker
            v-model:value="formValue.background_color"
            :show-alpha="true"
            :show-preview="true"
          />
        </n-form-item-gi>
      </n-grid>
    </n-form>
    <n-button attr-type="button" style="width: 100%" @click="handleAddNotice"> 确认添加 </n-button>
  </div>
</template>

<script setup lang="ts">
import '@vue-js-cron/naive-ui/dist/naive-ui.css';
import { CronNaive } from '@vue-js-cron/naive-ui';
import { FormInst } from 'naive-ui/es/form/src/interface';
import { invoke } from '@tauri-apps/api';
import { WebviewWindow, getCurrent } from '@tauri-apps/api/window';

const error = ref('');
const message = useMessage();
const formRef = ref<FormInst | null>(null);
const formValue = ref({
  scheduler: '* * * * *',
  notice_title: '',
  title_position: 'center',
  font_size: 20,
  title_color: '#000000',
  background_color: '#00000000',
});
const rules = {
  scheduler: {
    required: true,
    message: '设置触发时间',
    trigger: ['change'],
  },
  notice_title: {
    required: true,
    message: '请输入提示文字',
    trigger: ['input'],
  },
};

const noticeType = [
  {
    value: 'center',
    label: 'center',
  },
  {
    value: 'top',
    label: 'Top',
  },
  {
    value: 'bottom',
    label: 'Bottom',
  },
].map((s) => {
  s.value = s.value.toLowerCase();
  return s;
});

const handleAddNotice = (e: MouseEvent) => {
  formRef.value?.validate((errors) => {
    console.log(formValue.value);

    invoke('dispatch_command', {
      name: 'add_scheduler_job',
      args: {
        ...formValue.value,
        scheduler_id: generateUUID(),
        is_run: true,
      },
    }).then((response: any) => {
      console.log(response);

      message.success('添加成功');

      window.setTimeout(() => {
        const win = getCurrent();
        win.close();
        const mainWin = WebviewWindow.getByLabel('main');
        mainWin?.setFocus();
      }, 1500);
    });
  });
};

function generateUUID() {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
    var r = crypto.getRandomValues(new Uint8Array(1))[0] % 16 | 0;
    var v = c === 'x' ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}
</script>
<style scoped lang="scss">
.add_notice_page {
  width: 100vw;
  height: 100vh;
  background-color: rgb(40, 40, 40);
  position: fixed;
  top: 0;
  left: 0;
  padding: 20px;
  box-sizing: border-box;

  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
</style>
