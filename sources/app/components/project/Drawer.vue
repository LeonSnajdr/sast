<template>
    <VNavigationDrawer location="left" permanent>
        <VList class="mt-n2">
            <VListItem height="48">
                <template #prepend>
                    <VHover v-slot="{ isHovering, props }">
                        <VIcon @click="closeProject()" v-bind="props" :icon="isHovering ? 'mdi-folder' : 'mdi-folder-open'" color="info" />
                        <VTooltip activator="parent">{{ $t("project.drawer.close") }}</VTooltip>
                    </VHover>
                </template>
                <VListItemTitle>
                    <span class="font-weight-regular text-body-1">{{ selectedProject.name }}</span>
                </VListItemTitle>
                <template #append>
                    <IconBtn icon="mdi-swap-horizontal">
                        <ProjectSelectDialog />
                        <VTooltip activator="parent">{{ $t("project.drawer.select") }}</VTooltip>
                    </IconBtn>
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
            name: t("tabs.title"),
            to: { name: "index-project-id-tabs", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-label-outline",
            name: t("placeholder.title"),
            to: { name: "index-project-id-placeholder", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-checkbox-marked-circle-outline",
            name: t("task.title"),
            to: { name: "index-project-id-task", params: { id: selectedProject.value.id } }
        }
    ];
});

const closeProject = () => {
    navigateTo("/");
};
</script>
