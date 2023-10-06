import { createRouter, createWebHistory } from "vue-router";
import Home from "@/views/home/Home.vue";
import Settings from "@/views/settings/Settings.vue";
import Project from "@/views/project/Project.vue";
import TaskSet from "@/views/taskSet/TaskSet.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: Home,
            meta: {
                layout: "Project"
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
                layout: "Project"
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
