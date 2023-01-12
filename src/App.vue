<script setup lang="ts">
import { onMounted } from "@vue/runtime-core";
import { confirm } from '@tauri-apps/api/dialog';
import { exit } from '@tauri-apps/api/process';
import { appWindow } from '@tauri-apps/api/window';
onMounted(async () => {
  if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', (event) => event.preventDefault());
  }

  if (appWindow.label == 'main') {
    appWindow.onCloseRequested(async (event) => {
      const confirmed = await confirm('你确定退出吗?', { title: '提示', type: 'warning' });
      if (!confirmed) {
        event.preventDefault();
      } else {
        await exit(1);
      }
    });
  }
  document.onkeydown = function(event) {
    if(event.ctrlKey == true && event.code == 'KeyP') {
        console.log('Ctrl + p');
        // 或者 return false;
        event.preventDefault();
    }
    if(event.ctrlKey == true && event.code == 'KeyU') {
        console.log('Ctrl + u');
        // 或者 return false;
        event.preventDefault();
    }
    if(event.ctrlKey == true && event.code == 'KeyG') {
        console.log('Ctrl + g');
        // 或者 return false;
        event.preventDefault();
    }
}
});

</script>

<template>
  <router-view></router-view>
</template>

<style scoped>

</style>