import { createApp } from "vue";
import App from "./App.vue";
import "./titlebar.css"
import "virtual:uno.css"
import { appWindow } from '@tauri-apps/api/window'

const app = createApp(App)
app.mount("#app")

document
  .getElementById('titlebar-minimize')!
  .addEventListener('click', () => appWindow.minimize())
document
  .getElementById('titlebar-maximize')!
  .addEventListener('click', () => appWindow.toggleMaximize())
document
  .getElementById('titlebar-close')!
  .addEventListener('click', () => appWindow.close())