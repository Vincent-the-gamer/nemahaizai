<script setup lang="ts">
import { version } from "../../package.json"

const { locale } = useI18n()
</script>

<template>
    <h1 page-title>{{ $t("options") }}</h1>
    <p>
        <span>{{ $t("language") }}:</span>
        <span>
            <select select v-model="locale">
                <option value="en">English</option>
                <option value="zh_hans">简体中文(Simplified Chinese)</option>
            </select>
        </span>
    </p>
    <div absolute bottom-5>
        <p w-fit m="0 auto" translate-x--12>
            <p>2024-{{ $t("present") }} Vincent-the-gamer | {{ $t("version") }}: {{ version }}</p>
            <p w-fit m="0 auto">{{ $t("license") }}</p>
        </p>
    </div>
</template>