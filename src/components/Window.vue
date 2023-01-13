<template>
  <div class="win7">
  <div class="window glass active">
    <div data-tauri-drag-region class="title-bar">
      <div class="title-bar-text"><img src="../assets/vue.svg" style="width: 16px;height: 16px;border: 0px" />{{
        title
      }}</div>
      <div class="title-bar-controls">
        <button aria-label="Top" @click="onTop"><q-icon name="o_push_pin" color="white" :class="{'rotate-315':top}" size="1.4em"></q-icon></button>
        <button aria-label="Minimize" @click="onMinimize"></button>
        <button aria-label="Maximize" @click="onMaximize"></button>
        <button aria-label="Close" @click="onClose"></button>
      </div>
    </div>
    <div class="window-body has-space" style="height: calc(100vh - 40px);">
      <slot>Window 窗体内容</slot>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { confirm } from '@tauri-apps/api/dialog';
import { exit } from '@tauri-apps/api/process';
import { appWindow } from '@tauri-apps/api/window';
defineProps({
  title: String,
  icon: String
})
const top = ref(false);


const onTop = (e: Event) => {
  top.value = !top.value
  appWindow?.setAlwaysOnTop(top.value)
}
const onMinimize = (e: Event) => {
  appWindow?.minimize();
}
const onMaximize = (e: Event) => {
  appWindow?.toggleMaximize();
}
const onClose = async (e: Event) => {
  console.log(appWindow.label);
  if (appWindow.label == 'main') {
    const confirmed = await confirm('你确定退出吗?', { title: '提示', type: 'warning' });
    if (!confirmed) {
      e.preventDefault();
    } else {
      await exit(1);
    }
  } else {
    appWindow.close();
  }
}
</script>
<style lang="css" src="7.css/dist/7.scoped.css"  scoped>

</style>
<style scoped>
:deep(.window) {
  box-shadow: unset;
}
</style>