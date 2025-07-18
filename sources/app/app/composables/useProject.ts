export default function useProject() {
    const route = useRoute();
    const notify = useNotify();
    const { t } = useI18n();

    const isProjectLoading = ref(false);
    const project = ref<ProjectContract>({} as ProjectContract);

    const loadProject = async (taskId: string) => {
        isProjectLoading.value = true;

        const taskResult = await commands.projectGetOne(taskId);

        isProjectLoading.value = false;

        if (taskResult.status === "error") {
            notify.error(t("action.load.error", { type: t("project.singular") }), { error: taskResult.error });
            return;
        }

        project.value = taskResult.data;
    };

    const switchProject = async (project: ProjectContract) => {
        const isProjectRoute = route.matched.some((match) => match.name === "index-project-id");

        if (!isProjectRoute) return;

        const match = lodFindLast(route.matched, (match) => {
            return match.meta.projectSwitchName != undefined;
        });

        const name = match ? match.meta.projectSwitchName : "index-project-id";

        await navigateTo({ name: name as "index", params: { id: project.id } });
    };

    return { isProjectLoading, project, loadProject, switchProject };
}
