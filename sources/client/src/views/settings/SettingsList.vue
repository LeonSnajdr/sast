<template>
    <v-navigation-drawer width="200" permanent left floating>
        <template #prepend>
            <v-btn-icon icon="mdi-home" :to="{ name: 'home' }" />
        </template>
        <template #default>
            <v-divider />
            <v-list v-if="listProjects.length > 0">
                <v-list-item v-for="settingTab in settingTabs" :key="settingTab.title" :to="{ name: settingTab.value }" link>
                    <v-list-item-title>{{ settingTab.title }}</v-list-item-title>
                </v-list-item>
            </v-list>
            <span v-else>{{ $t("projectList.noItems") }}</span>
        </template>
    </v-navigation-drawer>
</template>

<script setup lang="ts">
import type { ListItem } from "vuetify/helpers";

const projectStore = useProjectStore();

const { listProjects } = storeToRefs(projectStore);

const settingTabs: ListItem[] = [
    {
        title: "General",
        value: "settingsGeneral"
    }
];

onBeforeMount(() => {
    projectStore.loadListProjects();
});
</script>

<style lang="scss" scoped>
.v-list-item {
    &--active {
        :deep(.v-icon) {
            color: white;
        }

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
