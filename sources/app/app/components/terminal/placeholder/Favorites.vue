<template>
    <div v-if="favoritePlaceholders.length > 0" class="d-flex flex-column ga-3">
        <div class="d-flex align-center ga-2 mb-2">
            <VIcon color="warning" icon="mdi-star" size="small" />
            <span class="text-medium-emphasis">{{ $t("terminal.favoritePlaceholders.title") }}</span>
        </div>
        <VRow>
            <VCol v-for="(placeholder, index) in favoritePlaceholders" :key="placeholder.id" cols="12" sm="6">
                <PlaceholderFieldAutosaveValue v-model="favoritePlaceholders[index]!" density="compact">
                    <template #label>
                        <PlaceholderIcon :visibility="placeholder.visibility" />
                        <span class="text-truncate">{{ placeholder.name }}</span>
                    </template>
                </PlaceholderFieldAutosaveValue>
            </VCol>
        </VRow>
    </div>
</template>

<script setup lang="ts">
const placeholderStore = usePlaceholderStore();

const { placeholders } = storeToRefs(placeholderStore);

const favoritePlaceholders = computed(() => {
    return placeholders.value.filter((placeholder) => placeholder.favorite);
});
</script>
