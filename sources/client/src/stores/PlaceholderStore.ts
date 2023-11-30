import type { Placeholder } from "@/bindings";
import * as commands from "@/bindings";

export const usePlaceholderStore = defineStore("PlaceholderStore", () => {
    const notify = useNotificationStore();
    const projectStore = useProjectStore();

    const { selectedProjectId } = storeToRefs(projectStore);

    const placeholders = ref<Placeholder[]>([]);

    const inPlaceholderEdit = ref(false);

    const loadPlaceholderList = async () => {
        try {
            placeholders.value = await commands.getPlaceholders(selectedProjectId.value);
        } catch (error) {
            console.error("Loading list projects failed", error);
            notify.error("TODO key");
        }
    };

    return {
        loadPlaceholderList,
        inPlaceholderEdit,
        placeholders
    };
});
