import { 
    defineConfig,
    presetUno, 
    presetAttributify,
    presetIcons
} from 'unocss'

export default defineConfig({
    presets: [
        presetUno(),
        presetAttributify(),
        presetIcons({
            extraProperties: {
                display: "inline-block"
            }
        })
    ],
    rules: [
        [
            /^bg-rgba-([\.\d]+)-([\.\d]+)-([\.\d]+)-([\.\d]+)$/, 
            ([_, r, g, b, a]) => ({ "background-color": `rgba(${r}, ${g}, ${b}, ${a})` })
        ],
        [
            /^font-family-([a-zA-Z_-]+)$/,
            ([_, fontFamily]) => ({ "font-family": fontFamily })
        ],
        [
            /^white-space-([a-zA-Z_-]+)$/,
            ([_, whiteSpace]) => ({ "white-space": whiteSpace })
        ],
        [
            /^\b(box|text)\b-shadow-([\.\d]+)px-([\.\d]+)px-([\.\d]+)px-([a-zA-Z_-]+)$/,
            ([_, type, offsetX, offsetY, blur, color]) => {
                if(type === "box") {
                    return {'box-shadow': `${offsetX}px ${offsetY}px ${blur}px ${color}`}
                } else {
                    return {'text-shadow': `${offsetX}px ${offsetY}px ${blur}px ${color}`}
                }
            }
        ],
        [
            "color-deeppink", { color: "deeppink"}
        ]
    ],
    shortcuts: [
        { "titlebar": "h-30px bg-#00dc82 select-none flex justify-end fixed top-0 left-0 right-0 z-10" },
        { "titlebar-button": "inline-flex justify-center items-center w-30px h-30px hover:bg-#59ffba hover:cursor-pointer" },
        { "icon": "font-size-8 color-black hover:color-white cursor-pointer" },
        { "page-title": "w-fit m-t-6 m-auto" },
        { "input": "p-2 border-rd-5px m-2 w-50 h-fit font-size-4"},
        { "select": "w-30 m-2 border-rd-5px font-size-4 bg-black color-white border-pink h-30px" }
    ]
})