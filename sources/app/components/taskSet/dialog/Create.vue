<template>
    <BaseDialogCreate
        v-model="isDialogOpen"
        v-model:element="taskSet"
        :emptyElement
        :type="$t('taskSet.singular')"
        icon="mdi-checkbox-multiple-marked-circle-outline"
    >
        <template #content>
            <TaskSetFieldContainer v-model="taskSet" v-model:isValid="isTaskSetValid" />
        </template>
        <template #actions>
            <TaskSetActionCreate @created="taskSetCreated" :disabled="!isTaskSetValid" :taskSet />
        </template>
    </BaseDialogCreate>
</template>

<script setup lang="ts">
const taskSet = ref({} as TaskSetCreateContract);
const isDialogOpen = ref(false);
const isTaskSetValid = ref<boolean | null>(false);

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const emptyElement: TaskSetCreateContract = {
    projectId: selectedProject.value.id,
    name: ""
};

const taskSetCreated = async (id: string) => {
    await navigateTo({ name: "index-project-id-taskSet-taskSetId", params: { id: selectedProject.value.id, taskSetId: id } });
    isDialogOpen.value = false;
};
</script>
