<script setup lang="ts">
import { version } from "../../package.json"
</script>

<template>
    <h1 page-title>Options</h1>
    <p>
        <span>Language:</span>
        <span>
            <select select>
                <option>English</option>
            </select>
        </span>
    </p>
    <div absolute bottom-5>
        <p w-fit m="l--30 auto">
            <p>2024-PRESENT Vincent-the-gamer | Version: {{ version }}</p>
            <p w-fit m="0 auto">MIT Licensed</p>
        </p>
        
    </div>
</template>