<template>
  <div class="container">
    <ConfigMenu />
    <div class="config_content">
      <div class="title">{{ title }}</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import ConfigMenu from './ConfigMenu.vue';
import { storeToRefs } from 'pinia';
import store from '../store';
import { computed } from 'vue';

const { files, selectedIndex } = storeToRefs(store());

const title = computed(() => {
  if(files.value!=null && selectedIndex.value!=null && files.value.length>0 && selectedIndex.value<files.value.length){
    return files.value[selectedIndex.value].name;
  }
  return "";
});

</script>

<style scoped>
.config_content{
  flex: 1;
  overflow: auto;
  border-radius: 10px;
  background-color: white;
  padding: 15px;
  margin-top: 10px;
}
.title{
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 20px;
}
.title_bar{
  display: flex;
  width: 100%;
  align-items: center;
}
.container{
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  display: flex;
  min-height: 0;
  flex-direction: column;
}

@media (prefers-color-scheme: dark) {
  .config_content{
    background-color: black;
  }
}
</style>