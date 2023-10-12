import type { FullProjectContract, ListProjectContract } from "@/bindings";
import { defineStore } from "pinia";
import { ref } from "vue";
import * as commands from "@/bindings";
import { useNotificationStore } from "./notificationStore";

export const useProjectStore = defineStore("project", () => {
    const notify = useNotificationStore();

    const listProjects = ref<ListProjectContract[]>([]);
    const project = ref<FullProjectContract>();

    const loadListProjects = async () => {
        try {
            listProjects.value = await commands.getListProjects();
        } catch (error) {
            console.error("Loading list projects failed", error);
            notify.error("projectStore.projectList.load.failed");
        }
    };

    const loadProject = async (projectId: string) => {
        try {
            project.value = await commands.getFullProject(projectId);
        } catch (error) {
            console.error("Loading project failed", error);
            notify.error("projectStore.project.load.failed");
        }
    };

    return { listProjects, project, loadProject, loadListProjects };
});
