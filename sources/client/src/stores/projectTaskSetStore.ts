import { TaskSetPageMode } from "@/enums/TaskSetPageMode";

export const useProjectTaskSetStore = defineStore("projectTaskSet", () => {
    const pageMode = ref<TaskSetPageMode>(TaskSetPageMode.View);

    return { pageMode };
});
