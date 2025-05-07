export const useProjectStore = defineStore("project", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const { project: selectedProject, loadProject } = useProject();

    const lastOpenedProject = ref<ProjectContract | null>();

    const isLoading = ref(false);

    const loadLastOpenedProject = async () => {
        const lastOpenedProjectResult = await commands.projectGetLastOpened();

        if (lastOpenedProjectResult.status == "error") {
            notify.error(t("project.load.failed"), { error: lastOpenedProjectResult.error });
            return;
        }

        lastOpenedProject.value = lastOpenedProjectResult.data;
    };

    return { isLoading, selectedProject, loadProject, lastOpenedProject, loadLastOpenedProject };
});
