

<template>
  <div class="app_wrap" data-tauri-drag-region>

    <button @click="handleClick"> 写入 </button>


    <button @click="handleClick2"> 打开 </button>

    <button @click="handleClick3"> 执行 </button>

    <button @click="handleClick4"> 事件 </button>


    <div>{{ result }}</div>


  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { appDataDir } from '@tauri-apps/api/path';
import { onMounted, ref } from 'vue';

const result = ref('');

const handleClick = () => {
  invoke('dispatch_command', { name: 'add_command', args: { cmd: 'sh', args: ["/xujunjie/aaa/cmd.sh"], current_dir: '/'}})
  .then((response) => console.log(response))
}
const handleClick2 = () => {
  invoke('dispatch_command', { name: 'get_all_commands', args: {}})
  .then((response) => console.log(response))
}

const handleClick3 = () => {
  invoke('dispatch_command', { name: 'execute_cmd', args: { cmd: 'ls', args: [] }})
  .then((response) => {
    console.log(response);
    result.value += JSON.stringify(response);
  })
}

const handleClick4 = () => {
  emit('click')
}


onMounted(() => {
  appDataDir().then((res) => {
    console.log("---", res)
  });
  
})

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
