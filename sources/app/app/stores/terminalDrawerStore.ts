export type DrawerFilter = "all" | "favorites" | "none";

export const useTerminalDrawerStore = defineStore(
    "terminalDrawer",
    () => {
        const isDrawerOpen = ref(false);
        const query = ref("");
        const taskFilter = ref<DrawerFilter>("none");
        const taskSetFilter = ref<DrawerFilter>("all");

        return { isDrawerOpen, query, taskFilter, taskSetFilter };
    },
    {
        persist: true
    }
);
