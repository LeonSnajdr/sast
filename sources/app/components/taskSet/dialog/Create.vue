<template>
    <BaseDialogCreate v-model="isDialogOpen" :emptyElement :type="$t('taskSet.singular')" icon="mdi-checkbox-multiple-marked-circle-outline">
        <template #fields="{ element }">
            <TaskSetFieldName v-model="element.name" autofocus />
        </template>
        <template #actions="{ isFormValid, element }">
            <TaskSetActionCreate @created="taskSetCreated" :disabled="!isFormValid" :taskSet="element" />
        </template>
    </BaseDialogCreate>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [id: string];
}>();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);

const emptyElement: TaskSetCreateContract = {
    projectId: selectedProject.value.id,
    name: ""
};

const taskSetCreated = (id: string) => {
    isDialogOpen.value = false;
    emit("created", id);
};
</script>
