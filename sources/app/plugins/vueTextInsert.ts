import vueTextInsert from "vue-text-insert";

export default defineNuxtPlugin((nuxtApp) => {
    nuxtApp.vueApp.use(vueTextInsert);
});
