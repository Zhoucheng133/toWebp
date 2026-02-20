<template>
  <div class="page">
    <v-btn icon="mdi-plus" variant="plain"></v-btn>
    <div class="label">添加一个图片文件，或者拖拽到这里</div>
  </div>
</template>

<script setup lang="ts">

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
  gap: 5px;
  font-size: 14px;
}

@media (prefers-color-scheme: dark) {
  .page{
    background-color: rgb(50, 50, 50);
  }
}
</style>