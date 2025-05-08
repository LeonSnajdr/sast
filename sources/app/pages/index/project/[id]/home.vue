<template>
    <VAppBar>
        <VAppBarTitle>
            {{ t("project.home") }}
        </VAppBarTitle>
    </VAppBar>
    <div>{{ selectedProject.id }}</div>
</template>

<script setup lang="ts">
const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

definePageMeta({
    projectSwitchName: "index-project-id-home"
});
</script>
