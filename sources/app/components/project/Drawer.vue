<template>
    <VNavigationDrawer location="left" width="240" permanent>
        <VList>
            <VListItem height="48">
                <VListItemTitle v-tooltip="selectedProject.name">
                    <span class="text-body-1">{{ selectedProject.name }}</span>
                </VListItemTitle>
                <template #append>
                    <BaseBtnIcon class="ml-1" icon="mdi-chevron-down" size="x-small" />
                    <BaseBtnIcon icon="mdi-swap-horizontal" size="x-small">
                        <ProjectDialogSelect />
                        <VTooltip activator="parent">{{ $t("project.drawer.select") }}</VTooltip>
                    </BaseBtnIcon>
                </template>
            </VListItem>
            <VDivider />
            <VListItem v-for="page in subPages" :key="page.name" :title="page.name" :to="page.to" link>
                <template #prepend="{ isActive }">
                    <VIcon :icon="isActive ? page.iconActive : page.icon" size="small" />
                </template>
            </VListItem>
        </VList>
        <template #append>
            <div class="pa-2 d-flex ga-2">
                <BaseBtnIcon class="flex-grow-1" icon="mdi-plus" variant="tonal">
                    {{ $t("action.create") }}
                    <ProjectDialogCreate />
                    <VTooltip activator="parent">{{ $t("project.drawer.create") }}</VTooltip>
                </BaseBtnIcon>
                <BaseBtnIcon @click="closeProject()" class="flex-grow-1" icon="mdi-close" variant="tonal">
                    {{ $t("action.close") }}
                </BaseBtnIcon>
            </div>
        </template>
    </VNavigationDrawer>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const subPages = computed((): { icon: string; iconActive: string; name: string; to: RouteLocationRaw }[] => {
    return [
        {
            icon: "mdi-home-outline",
            iconActive: "mdi-home",
            name: t("project.home"),
            to: { name: "index-project-id-home", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-tab",
            iconActive: "mdi-tab",
            name: t("terminal.plural"),
            to: { name: "index-project-id-terminal", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-label-outline",
            iconActive: "mdi-label",
            name: t("placeholder.plural"),
            to: { name: "index-project-id-placeholder", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-checkbox-marked-circle-outline",
            iconActive: "mdi-checkbox-marked-circle",
            name: t("task.plural"),
            to: { name: "index-project-id-task", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-checkbox-multiple-marked-circle-outline",
            iconActive: "mdi-checkbox-multiple-marked-circle",
            name: t("taskSet.plural"),
            to: { name: "index-project-id-taskSet", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-folder-cog-outline",
            iconActive: "mdi-folder-cog",
            name: t("projectSetting.singular"),
            to: { name: "index-project-id-settings", params: { id: selectedProject.value.id } }
        }
    ];
});

const closeProject = () => {
    navigateTo("/");
};
</script>
