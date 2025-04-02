export const useProjectStore = defineStore("project", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const lastOpenedProject = ref<ProjectContract | null>();

    const isLoading = ref(false);
    const selectedProject = ref<ProjectContract>({} as ProjectContract);

    const loadProject = async (projectId: string) => {
        isLoading.value = true;

        const projectResult = await commands.projectOpen(projectId);

        isLoading.value = false;

        if (projectResult.status == "error") {
            console.log(projectResult);

            notify.error(t("project.load.failed"), { error: projectResult.error });
            return;
        }

        selectedProject.value = projectResult.data;

        await loadLastOpenedProject();
    };

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
