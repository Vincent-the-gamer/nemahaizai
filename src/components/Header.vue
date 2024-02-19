<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'

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
        <div titlebar-button id="titlebar-minimize">
            <img src="/minimize.svg" alt="minimize" />
        </div>
        <div titlebar-button id="titlebar-maximize">
            <img src="/maximize.svg" alt="maximize" />
        </div>
        <div titlebar-button id="titlebar-close">
            <img src="/close.svg" alt="close" />
        </div>
    </div>
</template>