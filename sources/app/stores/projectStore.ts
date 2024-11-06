export const useProjectStore = defineStore("project", () => {
    const isLoading = ref(false);
    const selectedProject = ref<ProjectContract>({} as ProjectContract);

    return { isLoading, selectedProject };
});
