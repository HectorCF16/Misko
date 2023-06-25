const { invoke } = window.__TAURI__.tauri;

let passwordInputEl;
let warnMsgEl;
let ipMsgEl;

async function run_server() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  warnMsgEl.textContent = await invoke("run_server", { password: passwordInputEl.value });
}

async function get_ip() {
  ipMsgEl.textContent = await invoke("get_ip", {});
}

window.addEventListener("DOMContentLoaded", () => {
  passwordInputEl = document.querySelector("#password-input");
  warnMsgEl = document.querySelector("#warn-msg");
  ipMsgEl = document.querySelector("#ip-msg");
  get_ip();
  document
    .querySelector("#run-button")
    .addEventListener("click", () => run_server());
});
