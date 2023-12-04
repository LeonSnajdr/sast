import type { DefaultLocaleMessageSchema } from "vue-i18n";
import de from "./de.json";
import en from "./en.json";

const translations: { [key: string]: DefaultLocaleMessageSchema } = {
    de: de,
    en: en
};

export default translations;
