<template>
  Hello
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
onMounted(async ()=>{
  const appWindow = getCurrentWindow()
  appWindow.show();
})

</script>