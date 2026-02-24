<template>
  <div class="page">
    <div class="list_view">
      <div class="file_list_container">
        <FileItem v-for="file in files" :taskItem="file"></FileItem>
      </div>
    </div>
    <div class="config_view">
      <div class="config_container">

      </div>
    </div>
  </div>
</template>

<script setup lang="ts">

import { storeToRefs } from 'pinia';
import store from '../store';
import FileItem from '../components/FileItem.vue';

let { files } = storeToRefs(store());

</script>

<style scoped>
.config_container{
  height: 100%;
  width: 100%;
  background-color: blue;
  border-radius: 10px;
  /* background-color: rgb(245, 245, 245); */
}
.config_view{
  height: 100%;
  width: 100%;
  /* background-color: red; */
  padding-right: 20px;
  padding-bottom: 20px;
  box-sizing: border-box;
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
  /* padding-top: 10px; */
  overflow: hidden;
}
.page {
  padding: 0;
  display: grid;
  grid-template-columns: 300px auto;
  gap: 10px;
  height: 100vh;
  width: 100vw;
}
</style>