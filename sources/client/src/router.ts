import { createRouter, createWebHistory } from "vue-router";
import Settings from "@/views/settings/Settings.vue";
import Project from "@/views/project/Project.vue";
import TaskSet from "@/views/taskSet/TaskSet.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/:projectId?",
            name: "project",
            component: Project,
            props: (route) => ({
                projectId: route.params.projectId
            }),
            meta: {
                layout: "nav-left"
            }
        },
        {
            path: "/taskSet/:projectId/:taskSetId",
            name: "taskSet",
            component: TaskSet,
            props: (route) => ({
                projectId: route.params.projectId,
                taskSetId: route.params.taskSetId
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
