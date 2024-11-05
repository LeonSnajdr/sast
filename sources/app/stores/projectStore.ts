import { commands, type Project } from "~/types/tauri_bindings";

export const useProjectStore = defineStore("project", () => {
    const projects = ref<Project[]>([]);
    const selectedProjectId = ref<string>("");

    const loadProjects = async () => {
        const result = await commands.getAllProjects();

        if (result.status === "error") {
            return;
        }

        projects.value = result.data;
    };

    return { projects, selectedProjectId, loadProjects };
});
