import { path } from "@tauri-apps/api";
import { invoke } from "@tauri-apps/api/core";
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

  async function convertHandler(){
    running.value=true;

    for(let i=0;i<files.value!.length;i++){
      files.value![i].status=Status.processing;
      let outputPath=await path.join(output.value, `${await path.basename(files.value![i].name)}.webp`);

      console.log("【Run】"+i);
      

      const response: string=await invoke('convert', {
        path: files.value![i].path,
        width: files.value![i].width,
        height: files.value![i].height,
        quality: files.value![i].quality,
        output: outputPath,
      });

      if(response=='OK'){
        console.log("【Status】"+(files.value![i].status==Status.processing));
        files.value![i].status=Status.done;
        console.log("【Status】"+(files.value![i].status==Status.processing));
        
        console.log("【Done】"+i);
      }else{
        files.value![i].status=Status.err;
      }
    }

    running.value=false;
  }

  return {
    files,
    selectedIndex,
    output,
    running,
    convertHandler
  }
})