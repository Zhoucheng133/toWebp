<template>
  <div class="page">
    <div class="list_view">
      <div class="file_list_container">
        <FileItem v-for="(file, index) in files" :taskItem="file" :index="index"></FileItem>
      </div>
    </div>
    <div class="config_view">
      <Config />
    </div>
  </div>
</template>

<script setup lang="ts">

import { storeToRefs } from 'pinia';
import store from '../store';
import FileItem from '../components/FileItem.vue';
import Config from '../components/Config.vue';

let { files } = storeToRefs(store());

</script>

<style scoped>
/* .config_container{
  height: 100%;
  width: 100%;
  background-color: blue;
  border-radius: 10px;
} */
.config_view{
  height: 100%;
  width: 100%;
  /* background-color: red; */
  padding-right: 20px;
  padding-bottom: 20px;
  box-sizing: border-box;
  padding-top: 40px;
  min-height: 0;
}
.file_list_container{
  height: 100%;
  /* background-color: red; */
  overflow: auto;
}
.list_view{
  height: 100vh;
  width: 100%;
  /* background-color: orange; */
  box-sizing: border-box;
  padding-left: 20px;
  padding-bottom: 20px;
  padding-top: 40px;
  overflow: hidden;
}
.page {
  padding: 0;
  display: grid;
  grid-template-columns: 300px minmax(0, 1fr);
  gap: 10px;
  height: 100vh;
  width: 100vw;
}
</style>