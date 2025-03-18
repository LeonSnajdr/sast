export const usePtySessionStore = defineStore("ptySession", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const ptySessions = ref<PtySessionInfoContract[]>([]);

    const loadAll = async () => {
        isLoading.value = true;
        const sessionInfoResult = await commands.ptySessionGetManyInfo({ projectId: selectedProject.value.id, id: null, taskId: null, taskSetId: null });
        isLoading.value = false;

        if (sessionInfoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("ptySession.plural") }), { error: sessionInfoResult.error });
            return;
        }

        ptySessions.value = sessionInfoResult.data;
    };

    return { isLoading, ptySessions, loadAll };
});
