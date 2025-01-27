export const usePtySessionStore = defineStore("ptySession", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const ptySessions = ref<PtySessionInfoContract[]>([]);

    const loadAll = async () => {
        const sessionInfoResult = await commands.ptySessionInfoGetAll(selectedProject.value.id);

        if (sessionInfoResult.status === "error") {
            notify.error(t("ptySession.load.error"));
            return;
        }

        ptySessions.value = sessionInfoResult.data;
    };

    return { isLoading, ptySessions, loadAll };
});
