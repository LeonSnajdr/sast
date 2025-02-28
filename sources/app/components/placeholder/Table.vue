<template>
    <VDataTableVirtual :headers="headers" :items="placeholders" :rowProps="getRowProps" hideDefaultFooter>
        <template #[`item.dateCreated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateCreated) }}
        </template>
        <template #[`item.dateLastUpdated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateLastUpdated) }}
        </template>
        <template #[`item.actions`]>
            <BaseBtnIcon icon="mdi-pencil" />
        </template>
        <template #loading>
            <VSkeletonLoader type="table-row" />
        </template>
    </VDataTableVirtual>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";
import type { DataTableHeader } from "vuetify/helpers";

defineProps<{
    placeholders: PlaceholderContract[];
}>();

const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

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

const getRowProps = ({ item }: { item: PlaceholderContract }) => {
    return {
        onClick: () => {
            const placeholderRouteLoaction = getPlaceholderRouteLocation(item);
            navigateTo(placeholderRouteLoaction);
        }
    };
};

const getPlaceholderRouteLocation = (placeholder: PlaceholderContract): RouteLocationRaw => {
    return { name: "index-project-id-placeholder-placeholderId", params: { id: selectedProject.value.id, placeholderId: placeholder.id } };
};
</script>
