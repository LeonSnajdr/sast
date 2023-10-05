import { defineStore } from "pinia";
import { ref } from "vue";

export const useProjectListStore = defineStore(
    "projectList",
    () => {
        const rail = ref(false);
        return { rail };
    },
    {
        persist: { storage: localStorage }
    }
);
