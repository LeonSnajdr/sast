export const useTerminalStore = defineStore("terminal", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const terminals = ref<TerminalInfoContract[]>([]);

    const loadAll = async () => {
        isLoading.value = true;
        const sessionInfoResult = await commands.terminalGetManyInfo({ projectId: selectedProject.value.id, id: null, taskIds: null });
        isLoading.value = false;

        if (sessionInfoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("terminal.plural") }), { error: sessionInfoResult.error });
            return;
        }

        terminals.value = sessionInfoResult.data;
    };

    const statusChanged = async (id: string, status: TerminalShellStatus) => {
        const terminal = terminals.value.find((x) => x.id === id);

        if (terminal) {
            terminal.shellStatus = status;
        }
    };

    const deleted = async (id: string) => {
        terminals.value = terminals.value.filter((x) => x.id !== id);
    };

    return { isLoading, terminals, loadAll, statusChanged, deleted };
});
