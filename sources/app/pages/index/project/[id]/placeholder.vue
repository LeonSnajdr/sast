<template>
    <div class="fill-height d-flex flex-column">
        <VToolbar>
            <VToolbarTitle>
                <p class="text-caption">Platzhalter</p>
            </VToolbarTitle>
            <IconBtn color="secondary" icon="mdi-plus" variant="flat">
                <PlaceholderCreateDialog />
            </IconBtn>
        </VToolbar>
        <VContainer class="flex-grow-1 d-flex flex-column ga-4">
            <VCard title="Global">
                <template #prepend>
                    <VIcon color="info" icon="mdi-web" />
                </template>
                <VCardText>
                    <PlaceholderTable :placeholders="globalPlaceholders" />
                </VCardText>
            </VCard>
            <VCard title="Project">
                <template #prepend>
                    <VIcon color="success" icon="mdi-map-marker-radius" />
                </template>
                <VCardText>
                    <PlaceholderTable :placeholders="projectPlaceholders" />
                </VCardText>
            </VCard>
        </VContainer>
    </div>
</template>

<script setup lang="ts">
import { VDataTableVirtual } from "vuetify/components";
import type { DataTableHeader } from "vuetify/helpers";

const projectStore = useProjectStore();

const notify = useNotify();
const { t } = useI18n();

const { selectedProject } = storeToRefs(projectStore);

const isLoading = ref(false);
const globalPlaceholders = ref<PlaceholderContract[]>([]);
const projectPlaceholders = ref<PlaceholderContract[]>([]);

const headers: DataTableHeader[] = [
    {
        title: t("placeholder.table.column.name") as string,
        key: "name"
    },
    {
        title: t("placeholder.table.column.value") as string,
        key: "value"
    },
    {
        title: t("placeholder.table.column.dateCreated") as string,
        key: "dateCreated"
    },
    {
        title: t("placeholder.table.column.dateLastUpdated") as string,
        key: "dateLastUpdated"
    },
    {
        title: t("placeholder.table.column.actions") as string,
        key: "actions",
        width: 50
    }
];

onBeforeMount(() => {
    loadPlaceholders();
});

const loadPlaceholders = async () => {
    isLoading.value = true;

    const globalPlaceholdersResult = await commands.getGlobalPlaceholders();
    const projectPlaceholdersResult = await commands.getProjectPlaceholders(selectedProject.value.id);

    isLoading.value = false;

    if (globalPlaceholdersResult.status === "error" || projectPlaceholdersResult.status === "error") {
        notify.error(t("placeholder.load.error"));
        return;
    }

    globalPlaceholders.value = globalPlaceholdersResult.data;
    projectPlaceholders.value = projectPlaceholdersResult.data;
};
</script>
