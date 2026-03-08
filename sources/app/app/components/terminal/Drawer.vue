<template>
    <BaseDrawer v-model="isDrawerOpen">
        <template #title>
            {{ $t("terminal.plural") }}
        </template>
        <template #content>
            <div class="d-flex flex-column ga-4">
                <div class="d-flex flex-wrap ga-2 align-center">
                    <VTextField v-model="query" :placeholder="$t('search.filter')" density="compact" variant="plain" clearable hideDetails />
                    <VChipGroup v-model="visibleTypes" filter multiple>
                        <VChip density="comfortable" value="task">
                            {{ $t("task.plural") }}
                        </VChip>
                        <VChip density="comfortable" value="taskSet">
                            {{ $t("taskSet.plural") }}
                        </VChip>
                        <BaseBtnToggle
                            v-model="favoritesOnly"
                            :color="favoritesOnly ? 'warning' : 'secondary'"
                            :icon="favoritesOnly ? 'mdi-star' : 'mdi-star-outline'"
                            size="25"
                        />
                    </VChipGroup>
                </div>

                <div v-if="showTasks">
                    <div class="text-subtitle-2">
                        {{ $t("task.plural") }}
                    </div>
                    <TaskTable :tasks="filteredTasks" inline />
                </div>

                <div v-if="showTaskSets">
                    <div class="text-subtitle-2">
                        {{ $t("taskSet.plural") }}
                    </div>
                    <TaskSetTable :taskSets="filteredTaskSets" inline />
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

const isDrawerOpen = defineModel<boolean>({ required: true });

const taskStore = useTaskStore();
const taskSetStore = useTaskSetStore();

const { tasks } = storeToRefs(taskStore);
const { taskSets } = storeToRefs(taskSetStore);

const query = ref("");
const favoritesOnly = ref(false);
const visibleTypes = ref<VisibleType[]>(["task", "taskSet"]);

const normalizedQuery = computed(() => {
    return query.value.trim().toLowerCase();
});

const includesType = (type: VisibleType) => {
    return visibleTypes.value.includes(type);
};

const matchesFilters = (item: { name: string; favorite: boolean }) => {
    const hasMatchingName = normalizedQuery.value.length === 0 || item.name.toLowerCase().includes(normalizedQuery.value);
    const hasMatchingFavorite = !favoritesOnly.value || item.favorite;

    return hasMatchingName && hasMatchingFavorite;
};

const filteredTasks = computed(() => {
    if (!includesType("task")) return [];

    return tasks.value.filter(matchesFilters);
});

const filteredTaskSets = computed(() => {
    if (!includesType("taskSet")) return [];

    return taskSets.value.filter(matchesFilters);
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

const resetFilters = () => {
    query.value = "";
    favoritesOnly.value = false;
    visibleTypes.value = ["task", "taskSet"];
};

watch(isDrawerOpen, (isOpen, wasOpen) => {
    if (wasOpen && !isOpen) {
        resetFilters();
    }
});
</script>
