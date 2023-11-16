import type { TaskSet } from "@/bindings";
import * as commands from "@/bindings";

export const useTaskSetStore = defineStore("taskSetPage", () => {
    const notify = useNotificationStore();
    const projectStore = useProjectStore();

    const { project } = storeToRefs(projectStore);
    const taskSets = ref<TaskSet[]>([]);
    const executingTaskSets = ref<Record<string, boolean>>({});

    const loadTaskSets = async () => {
        try {
            taskSets.value = await commands.getTaskSets(project.value.id);
        } catch (error) {
            console.error("Loading task sets failed", error);
            // TODO Error message
        }
    };

    const startTaskSet = async (taskSetId: string) => {
        executingTaskSets.value[taskSetId] = true;

        try {
            await commands.startTaskSet(taskSetId);
        } catch (error) {
            console.error("Error while executing taskset", error);
            notify.error("taskSetView.execute.error");
        } finally {
            executingTaskSets.value[taskSetId] = false;
        }
    };

    const isExecuting = (taskSetId: string) => {
        return executingTaskSets.value[taskSetId];
    };

    return { taskSets, loadTaskSets, startTaskSet, isExecuting };
});
