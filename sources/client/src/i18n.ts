import { createI18n, type DefaultLocaleMessageSchema } from "vue-i18n";
import translations from "@/translations/index";

async function loadLocaleMessages(): Promise<{ [key: string]: DefaultLocaleMessageSchema }> {
    const localeMessages: { [key: string]: DefaultLocaleMessageSchema } = {};

    for (const localeKey in translations) {
        const messages = translations[localeKey];
        localeMessages[localeKey] = messages;
    }

    return localeMessages;
}

function getBrowserLanguage(): string {
    return window.navigator.language ? window.navigator.language.substring(0, 2) : "";
}

const i18n = createI18n({
    locale: getBrowserLanguage(),
    fallbackLocale: import.meta.env.VITE_DEFAULT_LANGUAGE,
    legacy: false,
    messages: await loadLocaleMessages()
});

export default i18n;
