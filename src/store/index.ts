import { defineStore } from "pinia";
import { ref } from "vue";

export enum Status{
  wait,
  processing,
  done,
  err,
}

export class TaskItem{
  public status: Status=Status.wait;
  public quality: number=80;
  public height: number=0;
  public width: number=0;
  constructor(
    public path: string,
    public name: string,
  ){}
}

export default defineStore("store", ()=>{

  const files=ref<null | TaskItem[]>(null);

  const selectedIndex=ref<null | number>(null);

  const output=ref("");

  const running=ref(false);

  return {
    files,
    selectedIndex,
    output,
    running,
  }
})