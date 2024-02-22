<script setup lang="ts">
const router = useRouter()

function toRoute(route: string) {
    router.push(route)
}
</script>

<template>
    <main flex="~ justify-center">
        <div absolute left-0 top-7 w-13 h-full class="bg-#00dc82" >
            <div flex="~ flex-col justify-center">
                <p absolute @click="() => toRoute('/')">
                    <div i-heroicons-musical-note-solid icon title="Audio Converter"/>
                </p>
                <p absolute top-12 @click="() => toRoute('/bpmDetect')">
                    <div i-heroicons-rectangle-stack-20-solid icon title="BPM Detect"/>
                </p>
                <p absolute bottom-4 @click="() => toRoute('/options')">
                    <div i-material-symbols-light-settings-outline icon title="Options"/>
                </p>
            </div>
        </div>
        <div>
            <RouterView/>
        </div>
    </main>
</template>