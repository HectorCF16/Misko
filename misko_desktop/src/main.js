const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("run_server", { password: greetInputEl.value });
}

async function get_ip() {
  await invoke("get_ip", {  }).then((response) => {
    document.querySelector("#ip").textContent = "Will listen at address: " + response + ":3333";
  });
}

window.addEventListener("DOMContentLoaded", () => {
  get_ip();
  greetInputEl = document.querySelector("#password-input");
  greetMsgEl = document.querySelector("#password-msg");
  document
    .querySelector("#password-button")
    .addEventListener("click", () => greet());
});
