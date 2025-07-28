export const useTaskSetStore = defineStore("taskSet", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const taskSets = ref<TaskSetInfoContract[]>([]);

    const loadAll = async () => {
        isLoading.value = true;

        const taskResult = await commands.taskSetGetManyInfo(selectedProject.value.id);

        isLoading.value = false;

        if (taskResult.status === "error") {
            notify.error(t("action.load.error", { type: t("taskSet.plural") }), { error: taskResult.error });
            return;
        }

        taskSets.value = taskResult.data;
    };

    return { isLoading, taskSets, loadAll };
});
