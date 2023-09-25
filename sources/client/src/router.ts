import { createRouter, createWebHistory } from "vue-router";
import Settings from "@/views/settings/Settings.vue";
import Project from "@/views/project/Project.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "project",
            component: Project
        },
        {
            path: "/settings",
            name: "settings",
            component: Settings
        }
    ]
});

export default router;
