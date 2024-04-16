<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
      <router-view v-slot="{ Component, route }">
        <keep-alive>
          <component :is="Component" v-if="route.meta.keepAlive" :key="route.name" />
        </keep-alive>
        <component :is="Component" v-if="!route.meta.keepAlive" :key="route.name" />
      </router-view>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { darkTheme } from 'naive-ui';
</script>
<style scoped lang="scss"></style>
