<script setup lang="ts">

const platform = getCurrentPlatform()

const paths = reactive({
    ncmInput: "",
    mp3Output: ""
})

const logs = ref<string[]>([])

onMounted(async () => {
    switch(platform) {
        case "windows":
            paths.ncmInput = "C:/Users/Public/ncm"
            paths.mp3Output = "C:/Users/Public/mp3"
            break
        case "macOS":
            paths.ncmInput = "/Users/Shared/ncm"
            paths.mp3Output = "/Users/Shared/mp3"
            break
        case "linux":
            // call Rust function to get current username of user's OS.
            const username = await invoke("whoami")

            paths.ncmInput = `/home/${username}/Public/ncm`
            paths.mp3Output = `/home/${username}/Public/mp3`
            break
        default: 
            break
    }
})


// call Rust function to convert .ncm audios to .mp3
async function ncm2mp3(){
    if(paths.ncmInput === "" || paths.mp3Output === ""){
        alert("paths can't be empty!")
    } else {
        try {
            await invoke("ncm2mp3", { 
                "ncmDir": paths.ncmInput,
                "outDir": paths.mp3Output
            })
            logs.value.push("Convert successful!")
        } catch(e) {
            logs.value.push("Convert error!")
        }
       
    }
}

</script>

<template>
    <h1 page-title>{{ $t("audio-converter") }}</h1>
    <div flex="~ col wrap justify-center items-center">
        <h2 m-1>{{ $t("ncm2mp3") }}</h2>
        <h4 m-1>{{ $t("description") }}</h4>
        <p m-1>
            <span>{{ $t("ncm-input-folder") }}:</span>
            <input input type="text" v-model="paths.ncmInput"/>
        </p>
        <p m-1>
            <span>{{ $t("mp3-output-folder") }}:</span>
            <input input type="text" v-model="paths.mp3Output"/>
        </p>
        <p m-1>
            <button button @click="ncm2mp3">{{ $t("convert") }}!</button>
        </p>
        <p m-1>{{ $t("log") }}: </p>
        <div m-1 w-full h-200px bg-rgba-34-34-34-0.5 border="rd-5px 2px solid yellow" overflow="y-auto x-hidden">
            <div relative h-full w-full flex="~ col wrap justify-center items-center">
                <p h-fit font-size-8 v-if="logs.length < 1">{{ $t("nothing") }}</p>
                <p h-fit font-size-8 v-else>
                    <p v-for="msg of logs">{{ msg }}</p>
                </p>
            </div>
        </div>
    </div>
</template>