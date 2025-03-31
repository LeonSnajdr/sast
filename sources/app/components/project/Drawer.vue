<template>
    <VNavigationDrawer location="left" permanent>
        <VList>
            <VListItem height="48">
                <VListItemTitle v-tooltip="selectedProject.name">
                    <span class="text-body-1">{{ selectedProject.name }}</span>
                </VListItemTitle>
                <template #append>
                    <BaseBtnIcon icon="mdi-plus" size="x-small">
                        <ProjectCreateDialog />
                        <VTooltip activator="parent">{{ $t("project.drawer.create") }}</VTooltip>
                    </BaseBtnIcon>
                    <BaseBtnIcon icon="mdi-swap-horizontal" size="x-small">
                        <ProjectSelectDialog />
                        <VTooltip activator="parent">{{ $t("project.drawer.select") }}</VTooltip>
                    </BaseBtnIcon>
                    <BaseBtnIcon @click="closeProject()" icon="mdi-close" size="x-small">
                        <VTooltip activator="parent">{{ $t("action.close") }}</VTooltip>
                    </BaseBtnIcon>
                </template>
            </VListItem>
            <VDivider />
            <VListItem v-for="page in subPages" :key="page.name" :title="page.name" :to="page.to" link>
                <template #prepend>
                    <VIcon :icon="page.icon" size="small" />
                </template>
            </VListItem>
        </VList>
    </VNavigationDrawer>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const subPages = computed((): { icon: string; name: string; to: RouteLocationRaw }[] => {
    return [
        {
            icon: "mdi-home",
            name: t("project.home"),
            to: { name: "index-project-id-home", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-tab",
            name: t("terminal.plural"),
            to: { name: "index-project-id-terminal", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-label-outline",
            name: t("placeholder.plural"),
            to: { name: "index-project-id-placeholder", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-checkbox-marked-circle-outline",
            name: t("task.plural"),
            to: { name: "index-project-id-task", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-checkbox-multiple-marked-circle-outline",
            name: t("taskSet.plural"),
            to: { name: "index-project-id-taskSet", params: { id: selectedProject.value.id } }
        }
    ];
});

const closeProject = () => {
    navigateTo("/");
};
</script>
