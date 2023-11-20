import type { FullProjectContract, ListProjectContract } from "@/bindings";
import * as commands from "@/bindings";

export const useProjectStore = defineStore("project", () => {
    const notify = useNotificationStore();

    const selectedProjectId = ref("");
    const listProjects = ref<ListProjectContract[]>([]);
    const project = ref<FullProjectContract>();

    const inPlaceholderEdit = ref(false);
    const inTaskSetEdit = ref(false);

    const runningTaskSets = ref<Record<string, boolean>>({});

    watch([selectedProjectId, inPlaceholderEdit, inTaskSetEdit], async () => {
        await loadProject();
    });

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

    const resetPageState = () => {
        inPlaceholderEdit.value = false;
        inTaskSetEdit.value = false;
    };

    const startTaskSet = async (taskSetId: string) => {
        runningTaskSets.value[taskSetId] = true;

        try {
            await commands.startTaskSet(taskSetId);
        } catch (error) {
            console.error("Error while executing taskset", error);
            notify.error("taskSetView.execute.error");
        } finally {
            runningTaskSets.value[taskSetId] = false;
        }
    };

    const isTaskSetRunning = (taskSetId: string) => {
        return runningTaskSets.value[taskSetId];
    };

    return {
        selectedProjectId,
        listProjects,
        project,
        inPlaceholderEdit,
        inTaskSetEdit,
        runningTaskSets,
        loadProject,
        loadListProjects,
        resetPageState,
        startTaskSet,
        isTaskSetRunning
    };
});
