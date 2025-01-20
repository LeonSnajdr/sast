<template>
    <VRowSingle>
        <ChipSelect v-model="projectId" :items="projectIdItems" itemText="translation" itemValue="projectId" />
    </VRowSingle>
</template>

<script setup lang="ts">
const projectId = defineModel<string | null>();

onMounted(() => {
    projectId.value = selectedProject.value.id;
});

const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const projectIdItems = computed(() => [
    {
        projectId: null,
        translation: t("placeholder.field.projectId.global")
    },
    {
        projectId: selectedProject.value.id,
        translation: t("placeholder.field.projectId.specific")
    }
]);
</script>
