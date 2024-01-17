<template>
    <v-navigation-drawer width="200" permanent left floating>
        <template #prepend>
            <div class="d-flex">
                <v-btn-icon :to="{ name: 'settingsGeneral' }" icon="mdi-cog" />
                <v-spacer />
                <v-btn-icon icon="mdi-plus">
                    <ProjectCreateDialog />
                </v-btn-icon>
            </div>

            <v-divider />
        </template>
        <template #default>
            <v-list v-if="listProjects.length > 0">
                <v-list-item v-for="listProject in listProjects" :key="listProject.id" :to="{ name: 'project', params: { projectId: listProject.id } }" link>
                    <v-list-item-title>{{ listProject.name }}</v-list-item-title>
                </v-list-item>
            </v-list>
            <span v-else>{{ $t("projectList.noItems") }}</span>
        </template>
    </v-navigation-drawer>
</template>

<script setup lang="ts">
import ProjectCreateDialog from "./ProjectCreateDialog.vue";

const projectStore = useProjectStore();

const { listProjects } = storeToRefs(projectStore);

onBeforeMount(() => {
    projectStore.loadListProjects();
});
</script>

<style lang="scss" scoped>
.v-list-item {
    :deep(.v-list-item__overlay) {
        opacity: 1;
        background-color: transparent;
    }

    &:hover {
        :deep(.v-list-item__overlay) {
            background-color: rgba(var(--v-theme-secondary), 0.2);
        }
    }

    &--active {
        :deep(.v-list-item__overlay) {
            background-color: rgba(var(--v-theme-secondary), 0.2);
            border-left: 3px solid rgb(var(--v-theme-primary));
        }
    }
}
</style>
