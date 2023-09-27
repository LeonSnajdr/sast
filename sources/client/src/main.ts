import "primevue/resources/themes/md-light-indigo/theme.css";
import "primevue/resources/primevue.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import primeVue from "primevue/config";
import toastService from "primevue/toastservice";

import App from "./App.vue";
import router from "./router";
import i18n from "./i18n";

import InputText from "primevue/inputtext";
import Button from "primevue/button";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18n);
app.use(primeVue);
app.use(toastService);

app.component("InputText", InputText);
app.component("Btn", Button);

app.mount("#app");
