<template>
  <div class="page">
    <div>
      <v-btn variant="flat" color="primary" @click="pickFile">选择图片文件</v-btn>
      <v-btn variant="plain" @click="pickDir">选择目录</v-btn>
    </div>
    <div class="label">或拖拽文件/目录到这里</div>
  </div>
</template>

<script setup lang="ts">
import { path } from '@tauri-apps/api';
import store from '../store';
import { storeToRefs } from 'pinia';
import { message } from '@tauri-apps/plugin-dialog';
import { onBeforeUnmount, onMounted } from 'vue';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { readDir } from '@tauri-apps/plugin-fs';

let unlisten: UnlistenFn;
let { files }= storeToRefs(store());

async function pickFile() {
  const file = await open({
    multiple: true,
    directory: false,
    filters: [
      {
        name: 'Images',
        extensions: ['jpeg', 'png', 'jpg']
      }
    ]
  });

  if(file){
    files.value = file;
  }
}

async function pickDir() {
  const selectedPath = await open({
    multiple: false,
    directory: true,
  });
  if (!selectedPath) return;
  const imageExtensions = ['jpg', 'jpeg', 'png'];
  const imageFiles = <string[]>[];
  const entries = await readDir(selectedPath);

  for (const entry of entries) {
    if (entry.isFile) {
      const name = entry.name;
      let ext;
      try {
        ext = (await path.extname(name)).toLowerCase();
      } catch (_) {
        continue;
      }
      
      if (ext && imageExtensions.includes(ext)) {
        imageFiles.push(await path.join(selectedPath, name));
      }
    }
  }
  files.value = imageFiles;

}

async function dropHandler(targets: string[]) {
  const firstPath=targets[0] as string;
  let extension;
  try {
    extension=(await path.extname(firstPath)).toLowerCase();
  } catch (_) {
    await message('不支持的文件', { title: '无法处理', kind: 'error' });
    return;
  }
  if(extension==='jpeg'||extension==='png'||extension==='jpg'){
    files.value = [firstPath];
  }else{
    await message('不支持的文件', { title: '无法处理', kind: 'error' });
  }
}

onMounted(async ()=>{
  unlisten = await listen('tauri://drag-drop', async (event: any) => {
    const payload = event?.payload;
    if (
      payload &&
      typeof payload === 'object' &&
      Array.isArray(payload.paths) &&
      typeof payload.paths[0] === 'string'
    ) {
      const targets = payload.paths;
      dropHandler(targets);
    }
  });
})

onBeforeUnmount(() => {
  if (unlisten) unlisten();
});

</script>

<style scoped>
.page{
  height: 100vh;
  width: 100vw;
  background-color: white;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  padding-bottom: 20px;
  user-select: none;
  -webkit-user-select: none;
  gap: 20px;
}
.label{
  font-size: 14px;
  color: rgb(170, 170, 170);
}

@media (prefers-color-scheme: dark) {
  .page{
    background-color: rgb(50, 50, 50);
  }
}
</style>