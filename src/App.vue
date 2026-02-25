<template>
  <AddView v-if="files==null" />
  <ConfigView v-else />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { storeToRefs } from 'pinia';
import store from './store';
import AddView from './views/AddView.vue';
import { useTheme } from 'vuetify';
import ConfigView from './views/ConfigView.vue';
const theme = useTheme();

onMounted(async ()=>{
  const appWindow = getCurrentWindow()
  appWindow.show();
  const systemTheme = await appWindow.theme();
  theme.change(systemTheme || 'light');
  await appWindow.listen('tauri://theme-changed', (event) => {
    theme.change(event.payload as string)
  })
  const output=localStorage.getItem('output');
  if(output){
    store().output=output;
  }
})

let { files }=storeToRefs(store());

</script>