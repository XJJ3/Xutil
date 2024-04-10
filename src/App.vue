

<template>
  <div class="app_window_wrap" @click="() => emit('blur')">
    <div class="app_menu_area">
      <button class="menu_item" @click.stop="handleClick">
        <img src="@/assets/menu/command.svg">
      </button>
    </div>
    <div class="app_cont_area" data-tauri-drag-region @click.stop="false">
      <Command></Command>
    </div>
    <div class="app_child_menu_area"></div>
  </div>
</template>

<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';
import Command from './components/Command.vue';

const handleClick = () => {
  console.log("12")
}
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

  user-select: none;
  -webkit-user-select: none;

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
      outline: none;
      color: white;
 
      display: flex;
      align-items: center;
      justify-content: center;

      border-radius: 8px;
      cursor: pointer;
      box-shadow: none;

      user-select: none;
  -webkit-user-select: none;
      

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
