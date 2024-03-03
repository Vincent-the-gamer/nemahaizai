import { defineConfig } from "vite"
import path from "path"
import vue from "@vitejs/plugin-vue"
import UnoCSS from "unocss/vite"
import AutoImport from "unplugin-auto-import/vite"
import Components from "unplugin-vue-components/vite"
import VueRouter from 'unplugin-vue-router/vite'
import { VueRouterAutoImports } from 'unplugin-vue-router'

export default defineConfig(async () => ({
  plugins: [
    vue(),
    UnoCSS(),
    AutoImport({
      imports: [
        "vue",
        "@vueuse/core",
        "vue-i18n",
        VueRouterAutoImports,
        {
          // add any other imports you were relying on
          'vue-router/auto': ['useLink'],
        },
        {
          from: "@vincent-the-gamer/utils/client",
          imports: ["detectBPM"]
        }
      ],
      dts: true,
      vueTemplate: true
    }),
    Components({
      dts: true,
      include: [/\.vue$/, /\.vue\?vue/]
    }),
    VueRouter()
  ],
  optimizeDeps: {
    include: [
      "vue"
    ],
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 8080,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  resolve: {
    alias: {
      "~": path.resolve(__dirname, "/src/")
    }
  }
}));
