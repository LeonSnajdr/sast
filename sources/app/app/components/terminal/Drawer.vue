<template>
    <BaseDrawer v-model="isDrawerOpen">
        <template #title>
            {{ $t("terminal.plural") }}
        </template>
        <template #content>
            <div class="d-flex flex-column ga-4">
                <VRow v-if="favoritePlaceholders.length > 0">
                    <VCol
                        v-for="(placeholder, index) in favoritePlaceholders"
                        :key="placeholder.id"
                        :sm="favoritePlaceholders.length % 2 === 1 && index === favoritePlaceholders.length - 1 ? 12 : 6"
                        cols="12"
                    >
                        <PlaceholderFieldAutosaveValue v-model="favoritePlaceholders[index]!" density="compact">
                            <template #label>
                                <PlaceholderIcon :visibility="placeholder.visibility" />
                                <span class="ml-1 text-truncate">{{ placeholder.name }}</span>
                            </template>
                        </PlaceholderFieldAutosaveValue>
                    </VCol>
                </VRow>
                <div class="d-flex flex-wrap ga-2 align-center">
                    <VTextField v-model="query" :placeholder="$t('search.filter')" density="compact" variant="plain" clearable />
                    <BaseChipCycle v-model="taskFilter" :states="filterStates" color="" density="comfortable" variant="tonal">
                        {{ $t("task.plural") }}
                    </BaseChipCycle>
                    <BaseChipCycle v-model="taskSetFilter" :states="filterStates" color="" density="comfortable" variant="tonal">
                        {{ $t("taskSet.plural") }}
                    </BaseChipCycle>
                </div>
                <div>
                    <TaskSetTable v-if="showTaskSets" :taskSets="filteredTaskSets" inline />
                    <TaskTable v-if="showTasks" :tasks="filteredTasks" inline />
                </div>
                <div v-if="!hasResults" class="text-medium-emphasis text-center">
                    {{ $t("search.noResults") }}
                </div>
            </div>
        </template>
    </BaseDrawer>
</template>

<script setup lang="ts">
import type { ChipCycleState } from "../base/chip/Cycle.vue";

const filterStates: ChipCycleState<DrawerFilter>[] = [
    { value: "all", icon: "mdi-check" },
    { value: "favorites", icon: "mdi-star", color: "warning" },
    { value: "none" }
];

const placeholderStore = usePlaceholderStore();
const taskStore = useTaskStore();
const taskSetStore = useTaskSetStore();
const terminalDrawerStore = useTerminalDrawerStore();

const { placeholders } = storeToRefs(placeholderStore);
const { tasks } = storeToRefs(taskStore);
const { taskSets } = storeToRefs(taskSetStore);
const { isDrawerOpen, query, taskFilter, taskSetFilter } = storeToRefs(terminalDrawerStore);

const { results: taskResults } = useFuse(query, tasks, {
    fuseOptions: {
        keys: ["name"],
        isCaseSensitive: false
    },
    matchAllWhenSearchEmpty: true
});

const { results: taskSetResults } = useFuse(query, taskSets, {
    fuseOptions: {
        keys: ["name"],
        isCaseSensitive: false
    },
    matchAllWhenSearchEmpty: true
});

const favoritePlaceholders = computed(() => {
    return placeholders.value.filter((placeholder) => placeholder.favorite);
});

const filteredTasks = computed(() => {
    if (taskFilter.value === "none") return [];

    return taskResults.value.map((x) => x.item).filter((task) => taskFilter.value !== "favorites" || task.favorite);
});

const filteredTaskSets = computed(() => {
    if (taskSetFilter.value === "none") return [];

    return taskSetResults.value.map((x) => x.item).filter((taskSet) => taskSetFilter.value !== "favorites" || taskSet.favorite);
});

const showTasks = computed(() => {
    return filteredTasks.value.length > 0;
});

const showTaskSets = computed(() => {
    return filteredTaskSets.value.length > 0;
});

const hasResults = computed(() => {
    return showTasks.value || showTaskSets.value;
});
</script>
