

<template>
  <div class="app_window_wrap" @click="() => emit('blur')">


    <div class="app_menu_area">
      <button class="menu_item">
        <img src="@/assets/menu/command.svg">
      </button>
    </div>


    <div class="app_cont_area" data-tauri-drag-region @click.stop="false">
        <button @click="handleClick"> 写入 </button>

        <button @click="handleClick2"> 打开 </button>

        <button @click="handleClick3"> 执行 </button>

        <button @click="handleClick4"> 事件 </button>

        <div>{{ result }}</div>
   

    </div>

    <div class="app_child_menu_area"></div>

    
  </div>

  <svg width="0" height="0" style="position:absolute;">
    <filter id="blur" color-interpolation-filters="sRGB">
      <feGaussianBlur stdDeviation="6" edgeMode="duplicate"/>
      <feComponentTransfer>
          <feFuncA type="discrete" tableValues="0 1"/>
      </feComponentTransfer>
    </filter>
</svg>
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
  emit('blur')
}


onMounted(() => {
  appDataDir().then((res) => {
    console.log("---", res)
  });
  
})

</script>

<style scoped lang="scss">
.app_window_wrap {
  width: 100vw;
  height: 100vh;
  background: rgba(255,0,0,0);
  position: fixed;
  top: 0;
  left: 0;
  color: white;
  display: flex;
  flex-wrap: wrap;

  .app_menu_area {
    width: calc(100% - 40px);
    height: 40px;
    // background-color: rgba(152, 138, 138, 0.1);
    display: flex;
    align-items: center;
    

    .menu_item {
      width: 32px;
      height: 32px;

      background-image: linear-gradient( 135deg, #d9d1d1 10%, #1904E5 100%);
      border: none;
      color: white;
 
      display: flex;
      align-items: center;
      justify-content: center;

      border-radius: 8px;
      cursor: pointer;
      background-size: 200%;
      transition: 0.6s;
      outline: none;

      img {
        width: 90%;
        height: 90%;
      }
    }
  }

  .app_cont_area {
    width: calc(100% - 40px);
    height: calc(100% - 40px);
    position: relative;
    background-color: rgba(0,0,0,0.2);
    border: 1px solid rgba(255,255,255,0.2);
    box-sizing: border-box;
    border-radius: 8px;
  }

  .app_child_menu_area {
    width: 40px;
    height: calc(100% - 40px);
    background-color: rgba(184, 135, 10, 0.1);
  }

}


</style>
