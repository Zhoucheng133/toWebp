<template>
  <AddView v-if="path==null" />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { storeToRefs } from 'pinia';
import store from './store';
import AddView from './views/AddView.vue';
import { useTheme } from 'vuetify';
const theme = useTheme();

onMounted(async ()=>{
  const appWindow = getCurrentWindow()
  appWindow.show();
  const systemTheme = await appWindow.theme();
  theme.change(systemTheme || 'light');
  await appWindow.listen('tauri://theme-changed', (event) => {
    theme.change(event.payload as string)
  })
})

let { path }=storeToRefs(store());

</script>