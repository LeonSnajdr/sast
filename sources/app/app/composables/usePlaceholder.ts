export default function usePlaceholder() {
    const notify = useNotify();
    const { t } = useI18n();

    const placeholderStore = usePlaceholderStore();

    const isPlaceholderSaving = ref(false);

    const savePlaceholder = async (placeholderToSave: PlaceholderContract) => {
        isPlaceholderSaving.value = true;

        const updateContract: PlaceholderUpdateContract = {
            id: placeholderToSave.id,
            projectId: placeholderToSave.projectId,
            name: placeholderToSave.name,
            favorite: placeholderToSave.favorite,
            value: placeholderToSave.value,
            visibility: placeholderToSave.visibility,
            kind: placeholderToSave.kind,
            source: placeholderToSave.source
        };

        const saveResult = await commands.placeholderUpdateOne(updateContract);

        isPlaceholderSaving.value = false;

        if (saveResult.status === "error") {
            notify.error(t("action.save.error", { type: t("placeholder.singular"), name: placeholderToSave.name }), { error: saveResult.error });
            return false;
        }

        await placeholderStore.loadAll();

        return true;
    };

    return { isPlaceholderSaving, savePlaceholder };
}
