type VisibleType = "task" | "taskSet";

export const useTerminalDrawerStore = defineStore(
    "terminalDrawer",
    () => {
        const isDrawerOpen = ref(false);
        const query = ref("");
        const favoritesOnly = ref(false);
        const visibleTypes = ref<VisibleType[]>(["task", "taskSet"]);

        return { isDrawerOpen, query, favoritesOnly, visibleTypes };
    },
    {
        persist: true
    }
);
