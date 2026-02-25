<template>
  <div class="menu">
    <v-btn variant="flat" color="primary" prepend-icon="mdi-play" @click="convert" :disabled="running">转换</v-btn>
    <v-btn variant="text" prepend-icon="mdi-cogs" @click="applyAll" :disabled="files?.length==1">应用到所有</v-btn>
    <v-btn variant="text" prepend-icon="mdi-close" style="margin-left: auto;" @click="close">关闭</v-btn>
  </div>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import store, { Status } from '../store';
import { confirm, message } from '@tauri-apps/plugin-dialog';
import { path } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/core';

const { files, selectedIndex, output, running } = storeToRefs(store());

async function convert(){
  if(output.value.length==0){
    await message('没有选择输出目录', { title: '错误', kind: 'error' });
    return;
  }

  running.value=true;

  for(let i=0;i<files.value!.length;i++){
    files.value![i].status=Status.processing;
    const fileNameWithExt = files.value![i].name;
    const ext = await path.extname(fileNameWithExt);
    const stem = await path.basename(fileNameWithExt, `.${ext}`);
    let outputPath=await path.join(output.value, `${stem}.webp`);

    console.log("【Run】"+i);
    

    const response: string=await invoke('convert', {
      path: files.value![i].path,
      width: files.value![i].width,
      height: files.value![i].height,
      quality: files.value![i].quality,
      output: outputPath,
    });

    if(response=='OK'){
      console.log("【Status】"+(files.value![i].status==Status.processing));
      files.value![i].status=Status.done;
      console.log("【Status】"+(files.value![i].status==Status.processing));
      
      console.log("【Done】"+i);
    }else{
      files.value![i].status=Status.err;
    }
  }

  running.value=false;
}

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