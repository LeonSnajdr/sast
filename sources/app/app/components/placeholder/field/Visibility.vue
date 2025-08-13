<template>
    <VRowSingle>
        <VBtnToggle v-model="visibility" @click.stop.prevent density="compact">
            <VBtn v-for="item of items" :key="item.visibility" :value="item.visibility">
                {{ item.translation }}
            </VBtn>
        </VBtnToggle>
    </VRowSingle>
</template>

<script setup lang="ts">
const visibility = defineModel<PlaceholderVisibility>({ required: true });
const projectId = defineModel<string | null>("projectId", { required: true });

const { t } = useI18n();
const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

watch(visibility, (newVisiblity) => {
    projectId.value = newVisiblity === "Project" ? selectedProject.value.id : null;
});

const items = computed((): { visibility: PlaceholderVisibility; translation: string }[] => [
    {
        visibility: "Global",
        translation: t("placeholder.field.visibility.global")
    },
    {
        visibility: "Project",
        translation: t("placeholder.field.visibility.project")
    }
]);
</script>
