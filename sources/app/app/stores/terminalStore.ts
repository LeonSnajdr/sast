export const useTerminalStore = defineStore(
    "terminal",
    () => {
        const notify = useNotify();
        const { t } = useI18n();

        const projectService = useProjectStore();

        const { selectedProject } = storeToRefs(projectService);

        const isLoading = ref(false);
        const allTerminals = ref<TerminalInfoContract[]>([]);

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

            allTerminals.value = sessionInfoResult.data;
        };

        const created = async (created: TerminalCreatedEventData) => {
            const infoResult = await commands.terminalGetOneInfo(created.id);

            if (infoResult.status === "error") {
                notify.error(t("action.load.error", { type: t("terminal.singular") }), { error: infoResult.error });
                return;
            }

            allTerminals.value.push(infoResult.data);

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

            lodAssign(terminal, infoResult.data);

            if (updated.jumpInto && updated.projectId === selectedProject.value.id) {
                navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: updated.id } });
            }
        };

        const statusChanged = async (update: TerminalShellStatusChangedEventData) => {
            const terminal = allTerminals.value.find((x) => x.id === update.id);

            if (!terminal) return;

            terminal.shellStatus = update.status;
        };

        const closed = async (id: string) => {
            allTerminals.value = allTerminals.value.filter((x) => x.id !== id);
        };

        return { isLoading, allTerminals, terminals, loadAll, created, updated, statusChanged, closed };
    }
);
