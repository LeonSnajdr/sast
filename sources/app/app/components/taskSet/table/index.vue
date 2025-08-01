<template>
    <VDataTableVirtual
        v-model:expanded="expandedTaskSetIds"
        :headers="headers"
        :hideDefaultHeader="inline"
        :items="taskSets"
        :rowProps="getRowProps"
        itemValue="id"
        hideDefaultFooter
        showExpand
    >
        <template #[`item.actions`]="{ item }">
            <TaskSetTableColumnActions :taskSet="item" />
        </template>
        <template #[`item.dateCreated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateCreated) }}
        </template>
        <template #[`item.dateLastUpdated`]="{ item }">
            {{ useLocaleTimeAgo(item.dateLastUpdated) }}
        </template>
        <template #[`item.data-table-expand`]="{ internalItem, isExpanded, toggleExpand }">
            <BaseBtnExpand @click.prevent.stop="toggleExpand(internalItem)" :modelValue="isExpanded(internalItem)" />
        </template>
        <template #expanded-row="{ columns, item }">
            <tr>
                <td :colspan="columns.length" class="pr-0">
                    <TaskSetSessionList :taskSetId="item.id" />
                </td>
            </tr>
        </template>
        <template #loading>
            <VSkeletonLoader type="table-row" />
        </template>
    </VDataTableVirtual>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";
import type { DataTableHeader } from "vuetify";

const props = defineProps<{
    taskSets: TaskSetInfoContract[];
    inline?: boolean;
}>();

const { t } = useI18n();

const projectStore = useProjectStore();
const taskSetSessionStore = useTaskSetSessionStore();

const { selectedProject } = storeToRefs(projectStore);
const { sessions } = storeToRefs(taskSetSessionStore);

const headers = computed((): DataTableHeader[] => {
    const defaultHeaders = [
        {
            title: t("taskSet.table.column.actions") as string,
            key: "actions",
            width: 150
        },
        {
            title: t("taskSet.table.column.name") as string,
            key: "name"
        }
    ];

    const additionalHeaders = [
        {
            title: t("taskSet.table.column.dateCreated") as string,
            key: "dateCreated"
        },
        {
            title: t("taskSet.table.column.dateLastUpdated") as string,
            key: "dateLastUpdated"
        }
    ];

    return props.inline ? defaultHeaders : [...defaultHeaders, ...additionalHeaders];
});

const getRowProps = ({ item }: { item: TaskSetInfoContract }) => {
    if (props.inline) return undefined;

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

const expandedTaskSetIds = ref<string[]>([]);

watch(
    sessions,
    () => {
        const runningTaskSetIds = sessions.value.filter((x) => x.status === "Running").map((x) => x.taskSetId);

        expandedTaskSetIds.value = lodUnion(expandedTaskSetIds.value, runningTaskSetIds);
    },
    { deep: true, immediate: true }
);
</script>
