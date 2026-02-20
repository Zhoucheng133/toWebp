import { defineStore } from "pinia";
import { ref } from "vue";

export default defineStore("store", ()=>{

  const path=ref<null | string>(null);

  return {
    path,
  }
})