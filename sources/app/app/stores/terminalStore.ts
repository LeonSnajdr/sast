export const useTerminalStore = defineStore("terminal", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const allTerminals = ref<TerminalInfoContract[]>([]);

    const pendingShellStatuses = new Map<string, TerminalShellStatus>();

    const terminals = computed(() => {
        return allTerminals.value.filter((x) => x.projectId === selectedProject.value.id);
    });

    const loadAll = async () => {
        isLoading.value = true;
        const sessionInfoResult = await commands.terminalGetManyInfo({} as TerminalFilter);
        isLoading.value = false;

        if (sessionInfoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("terminal.plural") }), { error: sessionInfoResult.error });
            return;
        }

        for (const fetched of sessionInfoResult.data) {
            const existing = allTerminals.value.find((x) => x.id === fetched.id);

            if (existing) {
                lodAssign(existing, { ...fetched, shellStatus: existing.shellStatus });
            } else {
                allTerminals.value.push(fetched);
            }
        }
    };

    const created = async (created: TerminalCreatedEventData) => {
        if (allTerminals.value.some((x) => x.id === created.id)) return;

        const infoResult = await commands.terminalGetOneInfo(created.id);

        if (infoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("terminal.singular") }), { error: infoResult.error });
            return;
        }

        const info = infoResult.data;
        const pendingStatus = pendingShellStatuses.get(created.id);
        if (pendingStatus !== undefined) {
            info.shellStatus = pendingStatus;
            pendingShellStatuses.delete(created.id);
        }

        if (allTerminals.value.some((x) => x.id === created.id)) return;

        allTerminals.value.push(info);

        if (created.jumpInto && created.projectId === selectedProject.value.id) {
            navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: created.id } });
        }
    };

    const updated = async (updated: TerminalUpdatedEventData) => {
        const infoResult = await commands.terminalGetOneInfo(updated.id);

        if (infoResult.status === "error") {
            notify.error(t("action.load.error", { type: t("terminal.singular") }), { error: infoResult.error });
            return;
        }

        const terminal = allTerminals.value.find((x) => x.id === updated.id);

        const info = { ...infoResult.data, shellStatus: terminal?.shellStatus ?? infoResult.data.shellStatus };

        lodAssign(terminal, info);

        if (updated.jumpInto && updated.projectId === selectedProject.value.id) {
            navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: updated.id } });
        }
    };

    const statusChanged = async (update: TerminalShellStatusChangedEventData) => {
        const terminal = allTerminals.value.find((x) => x.id === update.id);

        if (!terminal) {
            pendingShellStatuses.set(update.id, update.status);
            return;
        }

        terminal.shellStatus = update.status;
    };

    const closed = async (id: string) => {
        pendingShellStatuses.delete(id);
        allTerminals.value = allTerminals.value.filter((x) => x.id !== id);
    };

    return { isLoading, allTerminals, terminals, loadAll, created, updated, statusChanged, closed };
});
