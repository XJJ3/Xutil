<template>
  <div class="show_notice_page" @click="handleClose"></div>
</template>
<script setup lang="ts">
import { getCurrent } from '@tauri-apps/api/window';

const handleClose = () => {
  const win = getCurrent();
  win.close();
};
</script>
<style scoped lang="scss">
.show_notice_page {
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 1);
  position: fixed;
  top: 0;
  left: 0;
}
</style>
