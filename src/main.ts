import { invoke } from "@tauri-apps/api/tauri";

interface Entry {
  name: string;
  size: number;
}

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let listDirInputEl: HTMLInputElement | null;
let listDirMsgEl: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  listDirInputEl = document.querySelector("#list_dir-input");
  listDirMsgEl = document.querySelector("#list_dir-msg");
});

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

async function list_dir() {
  if (listDirInputEl && listDirMsgEl) {
    const result = await invoke("list_dir", {
      dir: listDirInputEl.value,
    }) as [Entry]
    listDirMsgEl.textContent = result.map(x => `${x.name}: ${x.size}`).join("|")
  }
}

window.greet = greet;
window.list_dir = list_dir;