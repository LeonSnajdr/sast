export const usePlaceholderStore = defineStore("placeholder", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const projectService = useProjectStore();

    const { selectedProject } = storeToRefs(projectService);

    const isLoading = ref(false);
    const placeholders = ref<PlaceholderContract[]>([]);

    const loadAll = async () => {
        isLoading.value = true;
        const placeholderResult = await commands.placeholderGetMany(selectedProject.value.id);
        isLoading.value = false;

        if (placeholderResult.status === "error") {
            notify.error(t("action.load.error", { type: t("placeholder.plural") }), { error: placeholderResult.error });
            return;
        }

        placeholders.value = placeholderResult.data;
    };

    return { isLoading, placeholders, loadAll };
});
