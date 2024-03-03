<script setup lang="ts">

</script>

<template>
    <h1 page-title>{{ $t("audio-converter") }}</h1>
    <div flex="~ col wrap justify-center items-center">
        <h2 m-1>{{ $t("ncm2mp3") }}</h2>
        <h4 m-1>{{ $t("description") }}</h4>
        <p m-1>
            <span>{{ $t("ncm-input-folder") }}:</span>
            <input input/>
        </p>
        <p m-1>
            <span>{{ $t("mp3-output-folder") }}:</span>
            <input input/>
        </p>
        <p m-1>
            <button button>{{ $t("convert") }}!</button>
        </p>
        <p m-1>{{ $t("log") }}: </p>
        <div m-1 w-full h-200px bg-rgba-34-34-34-0.5 border="rd-5px 2px solid yellow" overflow="y-auto x-hidden">
            <div relative h-full w-full flex="~ col wrap justify-center items-center">
                <p h-fit font-size-8>{{ $t("nothing") }}</p>
            </div>
        </div>
    </div>
</template>