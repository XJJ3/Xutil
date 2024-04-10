<template>
  <div class="command_wrapper">
    <!-- <button @click="handleClick"> 写入 </button>

    <button @click="handleClick2"> 打开 </button>

    <button @click="handleClick3"> 执行 </button>

    <button @click="handleClick4"> 事件 </button> -->

    <!-- <div>{{ result }}</div> -->


    <div class="small_btn"></div>
    <div class="small_btn"></div>
    <div class="small_btn"></div>
  </div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { emit } from '@tauri-apps/api/event';
import { ref } from 'vue';

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
  emit('blur')
}
</script>

<style scoped lang="scss">
.command_wrapper {
  width: 100%;
  padding: 10px;

  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;

  box-sizing: border-box;


  .small_btn {
    width: calc((100% - (12px * 2)) / 3);
    height: 32px;
    background-color: rgba(255,255,255,0.7);
    border-radius: 5px;
  }
}
</style>