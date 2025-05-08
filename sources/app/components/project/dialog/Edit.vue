<template>
    <BaseDialogEdit v-model="isDialogOpen" :type="$t('project.singular')" icon="mdi-folder">
        <template v-if="project.id" #content>
            <ProjectFieldContainer v-model="project" v-model:isValid="isValid" />
        </template>
        <template v-if="project.id" #actions>
            <ProjectActionSave @saved="projectSaved" :disabled="!isValid" :project />
        </template>
    </BaseDialogEdit>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    saved: [task: ProjectContract];
}>();

const props = defineProps<{
    projectId: string;
}>();

const isDialogOpen = ref(false);
const isValid = ref<boolean | null>(null);

const { project, loadProject } = useProject();

whenever(isDialogOpen, async () => {
    await loadProject(props.projectId);
});

const projectSaved = (project: ProjectContract) => {
    emit("saved", project);
    isDialogOpen.value = false;
};
</script>
