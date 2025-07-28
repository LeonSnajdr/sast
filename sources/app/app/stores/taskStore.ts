export const useTaskStore = defineStore("task", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const tasks = ref<TaskInfoContract[]>([]);

    const loadAll = async () => {
        isLoading.value = true;
        const taskResult = await commands.taskGetManyInfo(selectedProject.value.id);
        isLoading.value = false;

        if (taskResult.status === "error") {
            notify.error(t("action.load.error", { type: t("task.plural") }), { error: taskResult.error });
            return;
        }

        tasks.value = taskResult.data;
    };

    return { isLoading, tasks, loadAll };
});
