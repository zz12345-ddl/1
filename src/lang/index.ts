// index.ts
import { createI18n } from 'vue-i18n'
import zh from './zh_cn'


const messages = {
   
    zh
   
 
}

// 这是获取浏览器的语言
const language = navigator.language
const lang = language?.includes('zh') ? 'zh' : 'en'
const i18n = createI18n({
    // 首先从缓存里拿，没有的话就用浏览器语言，
    locale: localStorage.getItem('lang') || lang,
    fallbackLocale: 'zh', // 设置备用语言
    messages,
    legacy: false,
    globalInjection: true,
})

export default i18n
