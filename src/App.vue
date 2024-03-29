

<template>
  <div class="app_wrap" data-tauri-drag-region>

    <button @click="handleClick"> 打开 </button>

  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';

const handleClick = () => {
  invoke('dispatch_command', { name: 'command_one', args: {path: '111'}})
  .then((response) => console.log(response))
}
</script>

<style scoped>
.app_wrap {
  width: 100vw;
  height: 100vh;
  background: rgba(255,0,0,0.2);
  position: fixed;
  top: 0;
  left: 0;

  color: white;
}
</style>
