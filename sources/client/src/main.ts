import { createApp } from "vue";
import VRowSingle from "@/components/VRowSingle.vue";
import VButtonIcon from "@/components/VButtonIcon.vue";

import App from "./App.vue";
import router from "./router";
import vuetify from "./vuetify";
import i18n from "./i18n";
import pinia from "./pinia";

const app = createApp(App);

app.use(pinia);
app.use(vuetify);
app.use(router);
app.use(i18n);

app.component("VRowSingle", VRowSingle);
app.component("VButtonIcon", VButtonIcon);

app.mount("#app");
