<template>
    <VDataTableVirtual :headers="headers" :items="placeholders" hideDefaultFooter>
        <template #[`item.dateCreated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateCreated) }}
        </template>
        <template #[`item.dateLastUpdated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateLastUpdated) }}
        </template>
        <template #[`item.actions`]>
            <IconBtn icon="mdi-pencil" />
        </template>
    </VDataTableVirtual>
</template>

<script setup lang="ts">
import type { DataTableHeader } from "vuetify/helpers";

defineProps<{
    placeholders: PlaceholderContract[];
}>();

const { t } = useI18n();

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
</script>
