<template>
    <VNavigationDrawer location="left" permanent>
        <VList>
            <VListItem height="48">
                <VListItemTitle class="ga-1">
                    <VIcon v-if="selectedProject.favorite" color="warning" icon="mdi-star" />
                    <span class="text-body-1 text-truncate" style="max-width: 150px">{{ selectedProject.name }}</span>
                    <VIcon icon="mdi-chevron-down" />
                    <ProjectMenuSelect />
                </VListItemTitle>
                <template #append>
                    <VIconBtn id="swap-btn" class="ml-2" icon="mdi-swap-horizontal" size="small" v-tooltip="$t('project.drawer.select')" />
                    <ProjectDialogSelect activator="#swap-btn" />
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
            <div class="px-2 pb-2 d-flex ga-2">
                <VBtn class="flex-grow-1" variant="tonal">
                    <VIcon icon="mdi-plus" />
                    <VTooltip activator="parent">{{ $t("action.create") }}</VTooltip>
                    <ProjectDialogCreate />
                </VBtn>
                <VBtn :to="{ name: 'index-setting-index-presentation' }" class="flex-grow-1" variant="tonal">
                    <VIcon icon="mdi-cog" />
                    <VTooltip activator="parent">{{ $t("setting.title") }}</VTooltip>
                </VBtn>
            </div>
            <div class="px-2 pb-2 d-flex">
                <VBtn @click="closeProject()" class="flex-grow-1" prependIcon="mdi-close" variant="tonal">
                    {{ $t("action.close") }}
                </VBtn>
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
            to: { name: "index-project-id-setting", params: { id: selectedProject.value.id } }
        }
    ];
});

const closeProject = () => {
    navigateTo("/");
};
</script>
