import { defineStore } from "pinia";
import { ref } from "vue";
import { getListProjects, type ListProjectContract, getFullProject, type FullProjectContract } from "@/bindings";
import { useToast } from "primevue/usetoast";

export const useProjectStore = defineStore("sast-project", () => {
    const toast = useToast();

    const listProjects = ref<ListProjectContract[]>([]);
    const project = ref<FullProjectContract>();

    const loadListProjects = async () => {
        try {
            listProjects.value = await getListProjects();
        } catch (error) {
            console.error("Loading list projects failed", error);
            toast.add({ severity: "error", summary: "Error", detail: "Loading list projects failed", life: 3000 });
        }
    };

    const loadProject = async (projectId?: string) => {
        if (!projectId) {
            project.value = undefined;
            return;
        }

        try {
            const fullProject = await getFullProject(projectId);
            project.value = fullProject ?? undefined;
        } catch (error) {
            console.error("Loading project failed", error);
            toast.add({ severity: "error", summary: "Error", detail: "Loading project failed", life: 3000 });
        }
    };

    return { listProjects, loadListProjects, project, loadProject };
});
