import type { TaskSet } from "@/bindings";
import * as commands from "@/bindings";

export const useTaskSetStore = defineStore("taskSetStore", () => {
    const notify = useNotificationStore();
    const projectStore = useProjectStore();

    const { selectedProjectId } = storeToRefs(projectStore);

    const taskSets = ref<TaskSet[]>([]);

    const inTaskSetEdit = ref(false);
    const runningTaskSets = ref<Record<string, boolean>>({});

    const loadTaskSetList = async () => {
        try {
            taskSets.value = await commands.getTaskSets(selectedProjectId.value);
        } catch (error) {
            console.error("Loading task sets failed", error);
            notify.error("taskSetStore.load.taskSets.failed");
        }
    };

    const startTaskSet = async (taskSetId: string) => {
        runningTaskSets.value[taskSetId] = true;

        try {
            await commands.startTaskSet(taskSetId);
        } catch (error) {
            console.error("Error while executing task set", error);
            notify.error("taskSetStore.execute.error");
        } finally {
            runningTaskSets.value[taskSetId] = false;
        }
    };

    const isTaskSetRunning = (taskSetId: string) => {
        return runningTaskSets.value[taskSetId];
    };

    return {
        loadTaskSetList,
        startTaskSet,
        isTaskSetRunning,
        taskSets,
        inTaskSetEdit
    };
});
