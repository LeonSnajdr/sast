export const useProjectStore = defineStore("project", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const selectedProject = ref<ProjectContract>({} as ProjectContract);
    const lastOpenedProject = ref<ProjectContract | null>();
    const allProjects = ref<ProjectContract[]>([]);

    const isLoading = ref(false);
    const isLoadingAll = ref(false);

    const loadLastOpenedProject = async () => {
        const lastOpenedProjectResult = await commands.projectGetLastOpened();

        if (lastOpenedProjectResult.status == "error") {
            notify.error(t("project.load.failed"), { error: lastOpenedProjectResult.error });
            return;
        }

        lastOpenedProject.value = lastOpenedProjectResult.data;
    };

    const openProject = async (projectId: string) => {
        isLoading.value = true;
        const taskResult = await commands.projectOpen(projectId);
        isLoading.value = false;

        if (taskResult.status === "error") {
            notify.error(t("action.load.error", { type: t("project.singular") }), { error: taskResult.error });
            return;
        }

        selectedProject.value = taskResult.data;
    };

    const loadAllProjects = async () => {
        isLoadingAll.value = true;

        const projectsResult = await commands.projectGetAll();

        isLoading.value = false;

        if (projectsResult.status == "error") {
            notify.error(t("project.load.failed"), { error: projectsResult.error });
            return;
        }

        allProjects.value = projectsResult.data;
    };

    return { isLoading, selectedProject, lastOpenedProject, allProjects, loadAllProjects, openProject, loadLastOpenedProject };
});
