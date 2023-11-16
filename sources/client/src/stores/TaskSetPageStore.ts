import * as commands from "@/bindings";

export const useTaskSetPageStore = defineStore("taskSetPage", () => {
    const notify = useNotificationStore();

    const executingTaskSets = ref<Record<string, boolean>>({});

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

    return { startTaskSet, isExecuting };
});
