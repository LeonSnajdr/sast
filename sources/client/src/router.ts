import { createRouter, createWebHistory } from "vue-router";
import Settings from "@/views/settings/Settings.vue";
import Project from "@/views/project/Project.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/:projectId?",
            name: "project",
            component: Project,
            props: (route) => ({
                projectId: route.params.projectId
            })
        },
        {
            path: "/settings",
            name: "settings",
            component: Settings
        }
    ]
});

export default router;
