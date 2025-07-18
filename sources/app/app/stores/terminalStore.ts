export const useTerminalStore = defineStore("terminal", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const terminals = ref<TerminalInfoContract[]>([]);

    const loadAll = async () => {
        isLoading.value = true;
        const sessionInfoResult = await commands.terminalGetManyInfo({ projectId: selectedProject.value.id } as TerminalFilter);
        isLoading.value = false;

        if (sessionInfoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("terminal.plural") }), { error: sessionInfoResult.error });
            return;
        }

        terminals.value = sessionInfoResult.data;
    };

    const updated = async (id: string) => {
        const infoResult = await commands.terminalGetOneInfo(id);

        if (infoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("terminal.singular") }), { error: infoResult.error });
            return;
        }

        const terminal = terminals.value.find((x) => x.id === id);

        lodAssign(terminal, infoResult.data);
    };

    const statusChanged = async (update: TerminalShellStatusChangedEventData) => {
        const terminal = terminals.value.find((x) => x.id === update.id);

        if (!terminal) return;

        terminal.shellStatus = update.status;
    };

    const closed = async (id: string) => {
        terminals.value = terminals.value.filter((x) => x.id !== id);
    };

    return { isLoading, terminals, loadAll, updated, statusChanged, closed };
});
