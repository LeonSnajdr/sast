export const useTaskSetSessionStore = defineStore("taskSetSession", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const sessions = ref<TaskSetSessionContract[]>([]);

    const loadAll = async () => {
        isLoading.value = true;
        const sessionResult = await commands.taskSetSessionGetMany({ projectId: selectedProject.value.id } as TaskSetSessionFilter);
        isLoading.value = false;

        if (sessionResult.status === "error") {
            notify.error(t("action.load.error", { type: t("taskSetSession.plural") }), { error: sessionResult.error });
            return;
        }

        sessions.value = sessionResult.data;
    };

    const updated = async (id: string) => {
        const infoResult = await commands.taskSetSessionGetOne(id);

        if (infoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("taskSetSession.singular") }), { error: infoResult.error });
            return;
        }

        const terminal = sessions.value.find((x) => x.id === id);

        lodAssign(terminal, infoResult.data);
    };

    const taskStatusChanged = async (taskSetSessionId: string, taskId: string, status: TaskSetSessionTaskStatus) => {
        const task = sessions.value.find((x) => x.id === taskSetSessionId)?.tasks.find((x) => x.taskId === taskId);

        if (task) {
            task.status = status;
        }
    };

    return { isLoading, sessions, loadAll, updated, taskStatusChanged };
});
