<template>
  <div class="container">
    <ConfigMenu />
    <div class="config_content">
      <div class="title">{{ title }}</div>
      <div class="config_item mt-4">
        <div class="config_item_label">路径</div>
        <div class="config_item_child line-clamp-2">{{ files![selectedIndex!].path }}</div>
      </div>
      <div class="config_item mt-2">
        <div class="config_item_label">质量</div>
        <div class="config_item_child flex" style="padding-right: 20px; box-sizing: border-box; gap: 15px;">
          <v-slider v-model="files![selectedIndex!].quality" :step="10" :min="0" :max="100" style="flex: 1;" hide-details></v-slider>
          <v-text-field
            v-model="files![selectedIndex!].quality"
            density="compact"
            type="number"
            style="width: 90px; max-width: 90px;"
            variant="outlined"
            hide-details
          ></v-text-field>
        </div>
      </div>
      <div class="config_item mt-2">
        <div class="config_item_label">尺寸</div>
        <div class="config_item_child flex" style="gap: 15px; align-items: center;">
          <v-text-field
            v-model="files![selectedIndex!].width"
            density="compact"
            type="number"
            style="width: 120px; max-width: 150px;"
            variant="outlined"
            hide-details
          ></v-text-field>
          <v-icon icon="mdi-close"></v-icon>
          <v-text-field
            v-model="files![selectedIndex!].width"
            density="compact"
            type="number"
            style="width: 120px; max-width: 150px;"
            variant="outlined"
            hide-details
          ></v-text-field>
        </div>
      </div>
      <div class="config_item mt-2">
        <div class="config_item_label"></div>
        <div class="tip flex align-center gap-1">
          <i class="fa-solid fa-circle-info"></i>
          <p>0表示自动宽/高度，都为0则原始尺寸</p>
        </div>
      </div>
    </div>
    <div class="config_item" style="margin-top: 10px;">
      <div class="config_item_label">输出</div>
      <div class="config_item_child flex gap-1">
        <v-text-field v-model="output" density="compact" variant="outlined" hide-details></v-text-field>
        <v-btn variant="flat">选择</v-btn>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import ConfigMenu from './ConfigMenu.vue';
import { storeToRefs } from 'pinia';
import store from '../store';
import { computed } from 'vue';

const { files, selectedIndex, output } = storeToRefs(store());

const title = computed(() => {
  if(files.value!=null && selectedIndex.value!=null && files.value.length>0 && selectedIndex.value<files.value.length){
    return files.value[selectedIndex.value].name;
  }
  return "";
});

</script>

<style scoped>
.config_item_child{
  min-width: 0;
  text-align: left;
  width: 100%;
}
.config_item_label{
  width: 70px;
  flex-shrink: 0;
}
.config_item{
  display: flex;
  align-items: center;
}
.config_content{
  flex: 1;
  overflow: auto;
  border-radius: 10px;
  background-color: white;
  padding: 15px;
  margin-top: 10px;
  display: flex;
  flex-direction: column;
}
.title{
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