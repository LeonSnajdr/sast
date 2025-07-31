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

    const created = async (created: TerminalCreatedEventData) => {
        const infoResult = await commands.terminalGetOneInfo(created.id);

        if (infoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("terminal.singular") }), { error: infoResult.error });
            return;
        }

        terminals.value.push(infoResult.data);

        if (created.jumpInto) {
            navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: created.id } });
        }
    };

    const updated = async (updated: TerminalUpdatedEventData) => {
        const infoResult = await commands.terminalGetOneInfo(updated.id);

        if (infoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("terminal.singular") }), { error: infoResult.error });
            return;
        }

        const terminal = terminals.value.find((x) => x.id === updated.id);

        lodAssign(terminal, infoResult.data);

        if (updated.jumpInto) {
            navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: updated.id } });
        }
    };

    const statusChanged = async (update: TerminalShellStatusChangedEventData) => {
        const terminal = terminals.value.find((x) => x.id === update.id);

        if (!terminal) return;

        terminal.shellStatus = update.status;
    };

    const closed = async (id: string) => {
        terminals.value = terminals.value.filter((x) => x.id !== id);
    };

    return { isLoading, terminals, loadAll, created, updated, statusChanged, closed };
});
