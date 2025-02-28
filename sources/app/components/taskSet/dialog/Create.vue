<template>
    <BaseBtnIcon color="secondary" icon="mdi-plus" variant="flat">
        {{ $t("action.create") }}
        <BaseDialogCreate v-model="isDialogOpen" :emptyElement :type="$t('taskSet.singular')" icon="mdi-checkbox-multiple-marked-circle-outline">
            <template #fields="{ element }">
                <TaskSetFieldName v-model="element.name" />
            </template>
            <template #actions="{ isFormValid, element }">
                <TaskSetActionCreate @created="isDialogOpen = false" :disabled="!isFormValid" :taskSet="element" />
            </template>
        </BaseDialogCreate>
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);

const emptyElement: TaskSetCreateContract = {
    projectId: selectedProject.value.id,
    name: ""
};
</script>
