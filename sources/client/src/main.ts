import "primevue/resources/themes/md-light-indigo/theme.css";
import "primevue/resources/primevue.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import primeVue from "primevue/config";
import toastService from "primevue/toastservice";

import App from "./App.vue";
import router from "./router";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(primeVue);
app.use(toastService);

app.mount("#app");
