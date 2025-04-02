<template>
    <BaseDialogCreate v-model="isDialogOpen" :emptyElement :type="$t('project.singular')" icon="mdi-folder-plus">
        <template #actions="{ isFormValid, element }">
            <ProjectActionCreate @created="projectCreated" :disabled="!isFormValid" :project="element" />
        </template>
        <template #fields="{ element: project }">
            <ProjectFieldName v-model="project.name" autofocus />
        </template>
    </BaseDialogCreate>
</template>

<script setup lang="ts">
const isDialogOpen = ref(false);

const emptyElement: ProjectCreateContract = {
    name: ""
};

const projectCreated = (id: string) => {
    navigateTo({ name: "index-project-id-home", params: { id } });
};
</script>
