<template>
  <div class="win7">
  <div class="window glass active" style="box-shadow: unset;">
    <div data-tauri-drag-region class="title-bar">
      <div data-tauri-drag-region class="title-bar-text"><img data-tauri-drag-region :src="icon!=null?icon:'../src/assets/vue.svg'" style="width: 16px;height: 16px;border: 0px" />{{
        title
      }}</div>
      <div class="title-bar-controls">
        <button v-if="!hideTop" aria-label="Top" @click="onTop">
          <q-icon name="o_push_pin" color="white" :class="{'rotate-315':wintop}" size="1.2em" style="text-shadow: 0px 0px 2px black,1px 0px gray,0px 1px gray,-1px 0px gray,0px -1px gray;"></q-icon>
        </button>
        <button v-if="!hideMinimize" aria-label="Minimize" @click="onMinimize"></button>
        <button v-if="!hideMaximize" aria-label="Maximize" @click="onMaximize"></button>
        <button aria-label="Close" @click="onClose"></button>
      </div>
    </div>
    <div :class="['window-body',{'has-space':!noSpace},bgColor]" style="height: calc(100vh - 40px);">
      <slot>Window 窗体内容</slot>
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { confirm } from '@tauri-apps/api/dialog';
import { exit } from '@tauri-apps/api/process';
import { appWindow } from '@tauri-apps/api/window';
const props = defineProps<{
  title: string,
  icon?: string,
  bgColor?: string,
  noSpace?: boolean,
  hideTop?: boolean,
  hideMinimize?: boolean,
  hideMaximize?: boolean,
}>()
const wintop = ref(false);
appWindow.setResizable(!props.hideMaximize);
appWindow.setAlwaysOnTop(wintop.value);
watch(wintop, (newTop) => {
    appWindow.setAlwaysOnTop(newTop);
})

const onTop = (e: Event) => {
  wintop.value = !wintop.value
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
<style lang="css" src="7.css/dist/gui/window.css" scoped>

</style>
<style scoped>
:deep(.window) {
  box-shadow: unset;
}
html,body {
  background-color: transparent !important;
}
</style>