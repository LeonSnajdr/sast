import type { FullProjectContract, ListProjectContract } from "@/bindings";
import * as commands from "@/bindings";

export const useProjectStore = defineStore("projectStore", () => {
    const notify = useNotificationStore();

    const selectedProjectId = ref("");
    const listProjects = ref<ListProjectContract[]>([]);
    const project = ref<FullProjectContract>();

    const runningTaskSets = ref<Record<string, boolean>>({});

    const loadListProjects = async () => {
        try {
            listProjects.value = await commands.getListProjects();
        } catch (error) {
            console.error("Loading list projects failed", error);
            notify.error("projectStore.projectList.load.failed");
        }
    };

    const loadProject = async () => {
        try {
            project.value = await commands.getFullProject(selectedProjectId.value);
        } catch (error) {
            console.error("Loading project failed", error);
            notify.error("projectStore.project.load.failed");
        }
    };

    const isTaskSetRunning = (taskSetId: string) => {
        return runningTaskSets.value[taskSetId];
    };

    return {
        selectedProjectId,
        listProjects,
        project,
        runningTaskSets,
        loadProject,
        loadListProjects,
        isTaskSetRunning
    };
});
