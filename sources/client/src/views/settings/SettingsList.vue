<template>
    <v-navigation-drawer width="200" permanent left floating>
        <template #prepend>
            <v-btn-icon icon="mdi-home" @click="redirectBack" />
        </template>
        <template #default>
            <v-divider />
            <v-list>
                <v-list-item v-for="settingTab in settingTabs" :key="settingTab.title" :to="{ name: settingTab.value }" link>
                    <v-list-item-title>{{ settingTab.title }}</v-list-item-title>
                </v-list-item>
            </v-list>
        </template>
    </v-navigation-drawer>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { ListItem } from "vuetify/helpers";

const router = useRouter();
const i18n = useI18n();
const projectStore = useProjectStore();

const { selectedProjectId } = storeToRefs(projectStore);

const settingTabs = computed((): ListItem[] => [
    {
        title: i18n.t("settingsList.entry.general"),
        value: "settingsGeneral"
    },
    {
        title: i18n.t("settingsList.entry.appearance"),
        value: "settingsAppearance"
    }
]);

const redirectBack = () => {
    if (selectedProjectId.value) {
        router.push({ name: "project", params: { projectId: selectedProjectId.value } });
    } else {
        router.push({ name: "home" });
    }
};
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
