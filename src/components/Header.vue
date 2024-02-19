<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import Minimize from "~/assets/minimize.svg"
import Maximize from "~/assets/maximize.svg"
import Close from "~/assets/close.svg"
import Logo from "~/assets/logo.png"

onMounted(() => {
    document
        .getElementById('titlebar-minimize')!
        .addEventListener('click', () => appWindow.minimize())
    document
        .getElementById('titlebar-maximize')!
        .addEventListener('click', () => appWindow.toggleMaximize())
    document
        .getElementById('titlebar-close')!
        .addEventListener('click', () => appWindow.close())
})
</script>

<template>
    <div data-tauri-drag-region titlebar>
        <img :src="Logo" alt="logo" absolute left-0 h-7.5 />
        <div titlebar-button id="titlebar-minimize">
            <img :src="Minimize" alt="minimize" />
        </div>
        <div titlebar-button id="titlebar-maximize">
            <img :src="Maximize" alt="maximize" />
        </div>
        <div titlebar-button id="titlebar-close">
            <img :src="Close" alt="close" />
        </div>
    </div>
</template>