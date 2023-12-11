import { createRouter, createWebHistory } from "vue-router";
import ProjectLayout from "./layouts/ProjectLayout.vue";
import SettingsLayout from "./layouts/SettingsLayout.vue";
import Home from "@/views/home/Home.vue";
import SettingsGeneral from "@/views/settings/SettingsGeneral.vue";
import SettingsAppearance from "@/views/settings/SettingsAppearance.vue";
import Project from "@/views/project/Project.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: Home,
            meta: {
                layout: ProjectLayout
            }
        },
        {
            path: "/project/:projectId",
            name: "project",
            component: Project,
            props: (route) => ({
                projectId: route.params.projectId
            }),
            meta: {
                layout: ProjectLayout
            }
        },
        {
            path: "/settings/general",
            name: "settingsGeneral",
            component: SettingsGeneral,
            meta: {
                layout: SettingsLayout
            }
        },
        {
            path: "/settings/appearance",
            name: "settingsAppearance",
            component: SettingsAppearance,
            meta: {
                layout: SettingsLayout
            }
        }
    ]
});

export default router;
