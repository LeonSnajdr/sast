import DateFnsAdapter from "@date-io/date-fns";
import { enUS, enGB, de } from "date-fns/locale";

const locales: { [locale: string]: Locale } = {
    de,
    en: enUS,
    enUS,
    enGB
};

export function getLocale(): Locale {
    const inputLanguage = window.navigator.language;
    const defaultLangauge = import.meta.env.VITE_DEFAULT_LANGUAGE;

    const localeId: string = inputLanguage ? inputLanguage.replace("-", "") : "";
    const langId: string = inputLanguage ? inputLanguage.substring(0, 2) : "";

    return locales[localeId] || locales[langId] || locales[defaultLangauge] || enUS;
}

// use in vuetify https://vuetifyjs.com/en/features/dates/#usage
// supported formats https://github.com/dmtrKovalenko/date-io/blob/v2.17.0/packages/date-fns/src/date-fns-utils.ts#L60
export default new DateFnsAdapter({ locale: getLocale() });
