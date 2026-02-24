import { defineStore } from "pinia";
import { ref } from "vue";

enum Status{
  wait,
  processing,
  done
}

export class TaskItem{
  public status: Status=Status.wait;
  constructor(
    public path: string,
    public name: string,
  ){}
}

export default defineStore("store", ()=>{

  const files=ref<null | TaskItem[]>(null);

  const selectedIndex=ref<null | number>(null);

  return {
    files,
    selectedIndex
  }
})