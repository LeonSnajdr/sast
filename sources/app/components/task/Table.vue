<template>
    <VDataTableVirtual :headers="headers" :items="tasks" :rowProps="getRowProps" hideDefaultFooter>
        <template #[`item.dateCreated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateCreated) }}
        </template>
        <template #[`item.dateLastUpdated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateLastUpdated) }}
        </template>
        <template #[`item.actions`]>
            <IconBtn icon="mdi-pencil" />
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
    tasks: TaskInfoContract[];
}>();

const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const headers: DataTableHeader[] = [
    {
        title: t("task.table.column.name") as string,
        key: "name"
    },
    {
        title: t("task.table.column.blocking") as string,
        key: "blocking"
    },
    {
        title: t("task.table.column.dateCreated") as string,
        key: "dateCreated"
    },
    {
        title: t("task.table.column.dateLastUpdated") as string,
        key: "dateLastUpdated"
    },
    {
        title: t("task.table.column.actions") as string,
        key: "actions",
        width: 50
    }
];

const getRowProps = ({ item }: { item: TaskInfoContract }) => {
    return {
        onClick: () => {
            const taskRouteLoaction = getTaskRouteLocation(item);
            navigateTo(taskRouteLoaction);
        }
    };
};

const getTaskRouteLocation = (task: TaskInfoContract): RouteLocationRaw => {
    return { name: "index-project-id-task-taskId", params: { id: selectedProject.value.id, taskId: task.id } };
};
</script>
