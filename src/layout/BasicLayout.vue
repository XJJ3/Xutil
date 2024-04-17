<template>
  <div class="app_window_wrap" @click="() => emit('blur')">
    <div class="app_menu_area">
      <button class="menu_item" @click.stop="() => handleClick('/home/command')">
        <img src="@/assets/menu/command.svg" />
      </button>

      <button class="menu_item" @click.stop="() => handleClick('/home/translate')">
        <img src="@/assets/menu/translate.svg" />
      </button>
    </div>
    <div class="app_cont_area" data-tauri-drag-region @click.stop="false">
      <router-view v-slot="{ Component, route }">
        <keep-alive>
          <component :is="Component" v-if="route.meta.keepAlive" :key="route.name" />
        </keep-alive>
        <component :is="Component" v-if="!route.meta.keepAlive" :key="route.name" />
      </router-view>
    </div>
  </div>
</template>
<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';
import { useRouter } from 'vue-router';

const router = useRouter();

const handleClick = (route: string) => {
  router.push(route);
};
</script>
<style scoped lang="scss">
.app_window_wrap {
  width: 100vw;
  height: 100vh;
  background: rgba(255, 0, 0, 0);
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

      background-image: linear-gradient(135deg, #d9d1d1 10%, #1904e5 100%);
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
      padding: 7px;
      margin-right: 8px;

      &:last-of-type {
        margin-right: 0;
      }

      img {
        width: 100%;
        height: 100%;
      }
    }
  }

  .app_cont_area {
    width: 100%;
    height: calc(100% - 40px);
    position: relative;
  }
}
</style>
