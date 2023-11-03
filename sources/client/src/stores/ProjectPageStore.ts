import { defineStore } from "pinia";
import { ref } from "vue";

export const useProjectPageStore = defineStore("projectPage", () => {
    const inPlaceholderEdit = ref(false);
    const inTaskSetEdit = ref(false);

    const reset = () => {
        inPlaceholderEdit.value = false;
        inTaskSetEdit.value = false;
    };

    return { inPlaceholderEdit, inTaskSetEdit, reset };
});
