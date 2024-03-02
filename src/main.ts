import { createApp } from "vue";
import App from "./App.vue";
import "./main.css"
import "virtual:uno.css"
import { createRouter, createWebHistory } from 'vue-router/auto'
import i18n from "./i18n"

const app = createApp(App)

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
})

app.use(router)
app.use(i18n)
app.mount("#app")