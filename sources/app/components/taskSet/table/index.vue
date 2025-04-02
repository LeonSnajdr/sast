<template>
    <VDataTableVirtual :headers="headers" :items="taskSets" :rowProps="getRowProps" hideDefaultFooter>
        <template #[`item.actions`]="{ item }">
            <TaskSetTableColumnActions :taskSet="item" />
        </template>
        <template #[`item.dateCreated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateCreated) }}
        </template>
        <template #[`item.dateLastUpdated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateLastUpdated) }}
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
    taskSets: TaskSetInfoContract[];
}>();

const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const headers: DataTableHeader[] = [
    {
        title: t("taskSet.table.column.actions") as string,
        key: "actions",
        width: 200
    },
    {
        title: t("taskSet.table.column.name") as string,
        key: "name"
    },
    {
        title: t("taskSet.table.column.dateCreated") as string,
        key: "dateCreated"
    },
    {
        title: t("taskSet.table.column.dateLastUpdated") as string,
        key: "dateLastUpdated"
    }
];

const getRowProps = ({ item }: { item: TaskSetInfoContract }) => {
    return {
        onClick: () => {
            const taskSetRouteLoaction = getTaskSetRouteLocation(item);
            navigateTo(taskSetRouteLoaction);
        }
    };
};

const getTaskSetRouteLocation = (taskSet: TaskSetInfoContract): RouteLocationRaw => {
    return { name: "index-project-id-taskSet-taskSetId", params: { id: selectedProject.value.id, taskSetId: taskSet.id } };
};
</script>
