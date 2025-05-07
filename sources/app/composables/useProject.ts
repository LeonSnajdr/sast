export default function useProject() {
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

    return { isProjectLoading, project, loadProject };
}
