import { createRouter, createWebHistory } from "vue-router";
import ProjectLayout from "./layouts/ProjectLayout.vue";
import DefaultLayout from "./layouts/DefaultLayout.vue";
import Home from "@/views/home/Home.vue";
import Settings from "@/views/settings/Settings.vue";
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
            path: "/settings",
            name: "settings",
            component: Settings,
            meta: {
                layout: DefaultLayout
            }
        }
    ]
});

export default router;
