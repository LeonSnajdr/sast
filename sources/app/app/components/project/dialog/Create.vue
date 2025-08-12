<template>
    <BaseDialogCreate v-model="isDialogOpen" v-model:element="project" :emptyElement :type="$t('project.singular')" icon="mdi-folder-plus">
        <template #content>
            <ProjectFieldContainer v-model="project" v-model:isValid="isProjectValid" />
        </template>
        <template #actions>
            <ProjectActionCreate @created="projectCreated" :disabled="!isProjectValid" :project />
        </template>
    </BaseDialogCreate>
</template>

<script setup lang="ts">
const project = ref({} as ProjectCreateContract);
const isProjectValid = ref<boolean | null>(false);
const isDialogOpen = ref(false);

const emptyElement: ProjectCreateContract = {
    name: ""
};

const projectCreated = async (id: string) => {
    navigateTo({ name: "index-project-id", params: { id } });
};
</script>
