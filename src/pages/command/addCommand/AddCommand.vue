<template>
  <div class="add_command_page">
    <n-config-provider :theme="darkTheme">
      <n-form ref="formRef" :label-width="80" :model="formValue" :rules="rules">
        <n-form-item label="姓名" path="user.name">
          <n-input v-model:value="formValue.user.name" placeholder="输入姓名" />
        </n-form-item>
        <n-form-item label="年龄" path="user.age">
          <n-input v-model:value="formValue.user.age" placeholder="输入年龄" />
        </n-form-item>
        <n-form-item label="电话号码" path="phone">
          <n-input v-model:value="formValue.phone" placeholder="电话号码" />
        </n-form-item>
        <n-button attr-type="button" @click="handleValidateClick" style="width: 100%">
          确认添加
        </n-button>
      </n-form>
    </n-config-provider>
  </div>
</template>
<script setup lang="ts">
import { FormInst, useMessage } from 'naive-ui';
import { darkTheme } from 'naive-ui';

const message = useMessage();
const formRef = ref<FormInst | null>(null);
const formValue = ref({
  user: {
    name: '',
    age: '',
  },
  phone: '',
});
const rules = {
  user: {
    name: {
      required: true,
      message: '请输入姓名',
      trigger: 'blur',
    },
    age: {
      required: true,
      message: '请输入年龄',
      trigger: ['input', 'blur'],
    },
  },
  phone: {
    required: true,
    message: '请输入电话号码',
    trigger: ['input'],
  },
};

const handleValidateClick = (e: MouseEvent) => {
  e.preventDefault();
  formRef.value?.validate((errors) => {
    if (!errors) {
      message.success('Valid');
    } else {
      console.log(errors);
      message.error('Invalid');
    }
  });
};
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
