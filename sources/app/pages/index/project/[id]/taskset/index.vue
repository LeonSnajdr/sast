<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("taskset.plural") }}</VAppBarTitle>
    </VAppBar>
    <div class="fill-height d-flex flex-column">
        <VContainer>
            <VRowSingle>
                <VCard>
                    <VCardText>
                        <VDataTableVirtual :headers="headers" :items="tasks" :rowProps="getRowProps" hideDefaultFooter>
                            <template #[`item.tasksetActions`]>
                                <div class="d-flex">
                                    <IconBtn color="success" icon="mdi-play" />
                                    <IconBtn color="info" icon="mdi-autorenew" disabled />
                                    <IconBtn color="error" icon="mdi-stop" disabled />
                                </div>
                            </template>
                            <template #[`item.dragHandle`]>
                                <VIcon icon="mdi-drag" />
                            </template>
                            <template #[`item.dateCreated`]="{ item }">
                                {{ useLocaleTimeAgo(item.dateCreated) }}
                            </template>
                            <template #[`item.dateLastUpdated`]="{ item }">
                                {{ useLocaleTimeAgo(item.dateLastUpdated) }}
                            </template>
                            <template #[`item.taskCount`]="{ item }">
                                <VChip density="compact">{{ item.taskCount }}</VChip>
                            </template>
                            <template #[`item.actions`]>
                                <div class="d-flex">
                                    <IconBtn icon="mdi-pencil" />
                                </div>
                            </template>
                            <template #loading>
                                <VSkeletonLoader type="table-row" />
                            </template>
                        </VDataTableVirtual>
                    </VCardText>
                </VCard>
            </VRowSingle>
        </VContainer>
    </div>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";
import type { DataTableHeader } from "vuetify/helpers";

const tasks = ref([
    {
        name: "Yarn",
        dateCreated: "1.1.1",
        dateLastUpdated: "1.1.1",
        taskCount: 4
    }
]);

const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const headers: DataTableHeader[] = [
    {
        title: "",
        key: "dragHandle",
        sortable: false
    },
    {
        title: "Taskset aktionen",
        key: "tasksetActions",
        sortable: false
    },

    {
        title: t("placeholder.table.column.name") as string,
        key: "name"
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
        title: "taskCount",
        key: "taskCount"
    },
    {
        title: t("placeholder.table.column.actions") as string,
        key: "actions",
        sortable: false
    }
];

const getRowProps = ({ item }: { item: PlaceholderContract }) => {
    return {
        onClick: () => {
            const taskRouteLoaction = getTaskRouteLocation(item);
            navigateTo(taskRouteLoaction);
        }
    };
};

const getTaskRouteLocation = (task: PlaceholderContract): RouteLocationRaw => {
    return { name: "index-project-id-taskset-tasksetId", params: { id: selectedProject.value.id, tasksetId: "TODO" + task.name } };
};
</script>
