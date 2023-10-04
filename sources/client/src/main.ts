import "./style.css";

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
import Card from "primevue/card";

const DesignSystem = {
    card: {
        root: {
            class: ["p-4 bg-surface shadow-md rounded-md shadow-primary-lighten-3 "]
        },
        title: "text-2xl"
    },
    inputtext: {
        root: () => ({
            class: [
                "p-1 text-primary border-primary-lighten-2 border rounded-md hover:border-primary focus:border-primary focus:outline-none focus:outline-offset-0"
            ]
        })
    },
    button: {
        root: (props, context) => ({
            class: ["p-2 bg-primary rounded-md text-surface text-sm"]
        })
    }
};

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18n);
app.use(primeVue, { unstyled: true, pt: DesignSystem });
app.use(toastService);

app.component("InputText", InputText);
app.component("InputNumber", InputNumber);
app.component("Btn", Button);
app.component("Dropdown", Dropdown);
app.component("Card", Card);

app.mount("#app");
