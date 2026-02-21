import { defineStore } from "pinia";
import { ref } from "vue";

export default defineStore("store", ()=>{

  const files=ref<null | string[]>(null);

  return {
    files,
  }
})