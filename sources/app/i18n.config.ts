import en from "./locales/en";
import de from "./locales/de";

export default defineI18nConfig(() => {
    return {
        legacy: false,
        locale: "en",
        messages: {
            en,
            de
        }
    };
});
