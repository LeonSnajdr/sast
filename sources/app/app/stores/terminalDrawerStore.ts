type VisibleType = "task" | "taskSet";

export const useTerminalDrawerStore = defineStore(
    "terminalDrawer",
    () => {
        const isDrawerOpen = ref(false);
        const query = ref("");
        const taskFavoritesOnly = ref(false);
        const taskSetFavoritesOnly = ref(false);
        const visibleTypes = ref<VisibleType[]>(["taskSet"]);

        return { isDrawerOpen, query, taskFavoritesOnly, taskSetFavoritesOnly, visibleTypes };
    },
    {
        persist: true
    }
);
