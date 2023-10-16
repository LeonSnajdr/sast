<template>
    <v-navigation-drawer :rail="rail" @update:rail="(value) => (hover = !value)" width="200" permanent left floating expandOnHover>
        <template #default>
            <v-list>
                <v-list-item v-for="listProject in listProjects" :key="listProject.id" :to="{ name: 'project', params: { projectId: listProject.id } }" link>
                    <v-list-item-title>{{ listProject.name }}</v-list-item-title>
                </v-list-item>
            </v-list>
        </template>
        <template #append v-if="expanded">
            <div class="d-flex align-center pa-2">
                <v-btn color="primary" width="80%" density="comfortable">
                    <v-icon icon="mdi-plus" />
                    <ProjectCreateDialog />
                </v-btn>
                <v-spacer />
                <v-btn-icon @click.prevent="rail = !rail" :icon="rail ? 'mdi-pin-outline' : 'mdi-pin-off-outline'" />
            </div>
        </template>
    </v-navigation-drawer>
</template>

<script setup lang="ts">
import ProjectCreateDialog from "./ProjectCreateDialog.vue";

const pageStore = useProjectListStore();
const projectStore = useProjectStore();

const { rail } = storeToRefs(pageStore);
const { listProjects } = storeToRefs(projectStore);

const hover = ref(false);

onBeforeMount(() => {
    projectStore.loadListProjects();
});

const expanded = computed(() => {
    return !rail.value || hover.value;
});
</script>

<style lang="scss" scoped>
.v-list-item {
    &--active {
        :deep(.v-list-item__content) {
            z-index: 1;
            color: white;
        }

        :deep(.v-list-item__overlay) {
            opacity: 1;
            background-color: rgb(var(--v-theme-primary));
        }

        &:hover {
            :deep(.v-list-item__overlay) {
                opacity: 0.9;
            }
        }
    }
}
</style>
