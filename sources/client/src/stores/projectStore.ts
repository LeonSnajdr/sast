import type { ListProjectContract } from "@/bindings";
import * as commands from "@/bindings";

export const useProjectStore = defineStore("projectStore", () => {
    const notify = useNotificationStore();

    const selectedProjectId = ref("");
    const listProjects = ref<ListProjectContract[]>([]);

    const runningTaskSets = ref<Record<string, boolean>>({});

    const loadListProjects = async () => {
        try {
            listProjects.value = await commands.getListProjects();
        } catch (error) {
            console.error("Loading list projects failed", error);
            notify.error("projectStore.load.projectList.failed");
        }
    };

    return {
        selectedProjectId,
        listProjects,
        runningTaskSets,
        loadListProjects
    };
});
