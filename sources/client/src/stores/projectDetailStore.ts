import type { FullProjectContract } from "@/bindings";
import { defineStore } from "pinia";
import { ref } from "vue";
import * as commands from "@/bindings";
import { useNotificationStore } from "./notificationStore";

export const useProjectDetailStore = defineStore(
    "projectDetail",
    () => {
        const notify = useNotificationStore();

        const project = ref<FullProjectContract>();

        const load = async (projectId: string) => {
            try {
                project.value = await commands.getFullProject(projectId);
                notify.success("Keckw");
            } catch (error) {
                console.error("Loading project failed", error);
                notify.error("TODO");
            }
        };

        return { project, load };
    },
    {
        persist: { storage: localStorage }
    }
);
