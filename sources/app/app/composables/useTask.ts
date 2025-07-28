export default function useTask() {
    const notify = useNotify();
    const { t } = useI18n();

    const isTaskLoading = ref(false);
    const task = ref<TaskContract>();

    const loadTask = async (taskId: string) => {
        isTaskLoading.value = true;

        const taskResult = await commands.taskGetOne(taskId);

        isTaskLoading.value = false;

        if (taskResult.status === "error") {
            notify.error(t("action.load.error", { type: t("task.singular") }), { error: taskResult.error });
            return;
        }

        task.value = taskResult.data;
    };

    return { isTaskLoading, task, loadTask };
}
