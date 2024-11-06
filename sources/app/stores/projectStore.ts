import type { ProjectContract } from "~/types/tauriBindings";
import { commands } from "~/types/tauriBindings";

export const useProjectStore = defineStore("project", () => {
    const projects = ref<ProjectContract[]>([]);
    const selectedProjectId = ref<string>("");

    const loadProjects = async () => {
        const result = await commands.getAllProjects();

        console.log(result);

        if (result.status === "error") {
            console.error("failed to load projects");

            return;
        }

        projects.value = result.data;
    };

    return { projects, selectedProjectId, loadProjects };
});
