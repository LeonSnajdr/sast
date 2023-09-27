import { defineStore } from "pinia";
import { ref } from "vue";
import { getListProjects, type ListProjectContract } from "@/bindings";
import { useToast } from "primevue/usetoast";

export const useProjectStore = defineStore("sast-project", () => {
    const toast = useToast();

    const projectId = ref<string>();
    const listProjects = ref<ListProjectContract[]>([]);

    const loadListProjects = async () => {
        try {
            listProjects.value = await getListProjects();
        } catch (error) {
            console.error("Loading list projects failed", error);
            toast.add({ severity: "error", summary: "Error", detail: "Loading list projects failed", life: 3000 });
        }
    };

    return { projectId, listProjects, loadListProjects };
});
