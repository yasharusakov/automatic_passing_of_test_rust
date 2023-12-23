import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
    plugins: [react()],
    resolve: {
        alias: {
            "@assets": "/src/assets",
            "@styles": "/src/assets/styles",
            "@svgs": "/src/assets/svgs",
            "@components": "/src/components"
        }
    }
})