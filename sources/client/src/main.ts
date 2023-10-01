import "primevue/resources/themes/md-light-indigo/theme.css";
import "primevue/resources/primevue.css";
import "primeflex/primeflex.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import primeVue from "primevue/config";
import toastService from "primevue/toastservice";

import App from "./App.vue";
import router from "./router";
import i18n from "./i18n";

import InputText from "primevue/inputtext";
import InputNumber from "primevue/inputnumber";
import Button from "primevue/button";
import Dropdown from "primevue/dropdown";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18n);
app.use(primeVue);
app.use(toastService);

app.component("InputText", InputText);
app.component("InputNumber", InputNumber);
app.component("Btn", Button);
app.component("Dropdown", Dropdown);

app.mount("#app");
