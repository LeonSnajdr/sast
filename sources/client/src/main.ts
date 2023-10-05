import { createApp } from "vue";
import { createPinia } from "pinia";
import VRowSingle from "@/components/VRowSingle.vue";

import App from "./App.vue";
import router from "./router";
import vuetify from "./vuetify";
import i18n from "./i18n";

const app = createApp(App);

app.use(createPinia());
app.use(vuetify);
app.use(router);
app.use(i18n);

app.component("VRowSingle", VRowSingle);

app.mount("#app");
