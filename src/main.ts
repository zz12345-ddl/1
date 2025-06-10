import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia'
import i18n from './lang/index'
import { createGtag } from 'vue-gtag'
import '@/assets/fonts/iconfont.css'
import '@/assets/fonts/iconfont.js'
import 'element-plus/dist/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import '@/assets/theme.css'
import '@/assets/global.scss'
import 'vue-cropper/dist/index.css'

const app = createApp(App)
const pinia = createPinia()
app.use(router)
app.use(pinia)
app.use(i18n)
createGtag({
    tagId: 'G-7ZK1NBKC95',
})

app.mount('#app')
