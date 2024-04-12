<template>
  <n-message-provider>
    <router-view v-slot="{ Component, route }">
      <keep-alive>
        <component :is="Component" v-if="route.meta.keepAlive" :key="route.name" />
      </keep-alive>
      <component :is="Component" v-if="!route.meta.keepAlive" :key="route.name" />
    </router-view>
  </n-message-provider>
</template>

<script setup lang="ts"></script>
<style scoped lang="scss"></style>
