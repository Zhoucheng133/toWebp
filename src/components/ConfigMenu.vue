<template>
  <div class="menu">
    <v-btn variant="flat" color="primary" prepend-icon="mdi-play">转换</v-btn>
    <v-btn variant="text" prepend-icon="mdi-cogs">应用到所有</v-btn>
    <v-btn variant="text" prepend-icon="mdi-close" style="margin-left: auto;" @click="close">关闭</v-btn>
  </div>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import store from '../store';

const { files, selectedIndex } = storeToRefs(store());

function close(){
  files.value=null;
  selectedIndex.value = null;
}
</script>

<style scoped>
.menu{
  display: flex;
  gap: 10px;
}
</style>