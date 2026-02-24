<template>
  <div class="container">
    <div class="file_name">{{ taskItem.name }}</div>
    <div class="file_path">{{ taskItem.path }}</div>
  </div>
</template>

<script lang="ts" setup>
import { TaskItem } from '../store';

const props = defineProps(["taskItem"])
const taskItem = props.taskItem as TaskItem;


</script>

<style scoped>
.file_path{
  width: 100%;
  overflow: hidden;
  max-lines: 1;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
}
.container {
  display: flex;
  align-items: flex-start;
  flex-direction: column;
  justify-content: center;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  background-color: lightgrey;
  box-sizing: border-box;
  /* padding: 7px; */
  padding-top: 7px;
  padding-bottom: 7px;
  padding-left: 10px;
  padding-right: 10px;
  border-radius: 10px;
  margin-bottom: 10px;
}
</style>