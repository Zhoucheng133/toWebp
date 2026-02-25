<template>
  <div class="menu">
    <v-btn variant="flat" color="primary" prepend-icon="mdi-play">转换</v-btn>
    <v-btn variant="text" prepend-icon="mdi-cogs" @click="applyAll" :disabled="files?.length==1">应用到所有</v-btn>
    <v-btn variant="text" prepend-icon="mdi-close" style="margin-left: auto;" @click="close">关闭</v-btn>
  </div>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import store from '../store';
import { confirm } from '@tauri-apps/plugin-dialog';

const { files, selectedIndex } = storeToRefs(store());

async function applyAll(){
  const confirmation = await confirm(
    '确定要应用这个配置到所有的文件吗',
    { title: '应用到所有', kind: 'info', okLabel: '确定', cancelLabel: '取消' }
  );
  if(confirmation){
    for(let i=0;i<files.value!!.length;i++){
      files.value![i].quality = files.value![selectedIndex.value!].quality;
      files.value![i].width = files.value![selectedIndex.value!].width;
      files.value![i].height = files.value![selectedIndex.value!].height;
    }
  }
}

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