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
                    <VChipGroup v-model="visibleTypes" filter multiple>
                        <VChip density="comfortable" value="task">
                            {{ $t("task.plural") }}
                            <template #append>
                                <span @click.stop>
                                    <BaseBtnToggle
                                        v-model="taskFavoritesOnly"
                                        :color="taskFavoritesOnly ? 'warning' : 'secondary'"
                                        :icon="taskFavoritesOnly ? 'mdi-star' : 'mdi-star-outline'"
                                        size="25"
                                    />
                                </span>
                            </template>
                        </VChip>
                        <VChip density="comfortable" value="taskSet">
                            {{ $t("taskSet.plural") }}
                            <template #append>
                                <span @click.stop>
                                    <BaseBtnToggle
                                        v-model="taskSetFavoritesOnly"
                                        :color="taskSetFavoritesOnly ? 'warning' : 'secondary'"
                                        :icon="taskSetFavoritesOnly ? 'mdi-star' : 'mdi-star-outline'"
                                        size="25"
                                    />
                                </span>
                            </template>
                        </VChip>
                    </VChipGroup>
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
type VisibleType = "task" | "taskSet";

const placeholderStore = usePlaceholderStore();
const taskStore = useTaskStore();
const taskSetStore = useTaskSetStore();
const terminalDrawerStore = useTerminalDrawerStore();

const { placeholders } = storeToRefs(placeholderStore);
const { tasks } = storeToRefs(taskStore);
const { taskSets } = storeToRefs(taskSetStore);
const { isDrawerOpen, query, taskFavoritesOnly, taskSetFavoritesOnly, visibleTypes } = storeToRefs(terminalDrawerStore);

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

const includesType = (type: VisibleType) => {
    return visibleTypes.value.includes(type);
};

const favoritePlaceholders = computed(() => {
    return placeholders.value.filter((placeholder) => placeholder.favorite);
});

const filteredTasks = computed(() => {
    if (!includesType("task")) return [];

    return taskResults.value.map((x) => x.item).filter((task) => !taskFavoritesOnly.value || task.favorite);
});

const filteredTaskSets = computed(() => {
    if (!includesType("taskSet")) return [];

    return taskSetResults.value.map((x) => x.item).filter((taskSet) => !taskSetFavoritesOnly.value || taskSet.favorite);
});

const showTasks = computed(() => {
    return includesType("task") && filteredTasks.value.length > 0;
});

const showTaskSets = computed(() => {
    return includesType("taskSet") && filteredTaskSets.value.length > 0;
});

const hasResults = computed(() => {
    return showTasks.value || showTaskSets.value;
});
</script>
