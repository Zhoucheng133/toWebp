<template>
  <div :class="index==selectedIndex ? 'container_selected' : 'container' " @click="selectTask">
    <div class="status">
      <i class="fa-regular fa-clock" v-if="files![selectedIndex!].status==Status.wait"></i>
      <i class="fa-solid fa-hourglass" v-if="files![selectedIndex!].status==Status.processing"></i>
      <i class="fa-solid fa-check" v-if="files![selectedIndex!].status==Status.done"></i>
    </div>
    <div class="info">
      <div class="file_name">{{ taskItem.name }}</div>
      <div class="file_path">{{ taskItem.path }}</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { Status, TaskItem } from '../store';
import store from '../store';

const { selectedIndex, files } = storeToRefs(store());

const props = defineProps(["taskItem", "index"])
const taskItem = props.taskItem as TaskItem;
const index = props.index as number;

function selectTask(){
  selectedIndex.value = index;
}

</script>

<style scoped>
.info{
  min-width: 0;
}
.file_name{
  font-size: 15px;
}
.file_path{
  width: 100%;
  overflow: hidden;
  max-lines: 1;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
}

.container_selected{
  background-color: #E9F3FE;
}

.container:hover{
  background-color: #EDF5FE;
}

@media (prefers-color-scheme: dark) {
  .container_selected{
    background-color: rgb(80, 80, 80);
  }

  .container:hover{
    background-color: rgb(50, 50, 50);
  }
}

.container, .container_selected {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;

  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  transition: background-color 0.2s linear;
  box-sizing: border-box;
  padding-top: 7px;
  padding-bottom: 7px;
  padding-left: 10px;
  padding-right: 10px;
  border-radius: 10px;
  margin-bottom: 5px;
}
</style>