<template>
    <VMenu v-model="open" :target="menu.value.position">
        <VList>
            <VListItem v-for="placeholder in filteredPlaceholders" :key="placeholder.id" @click="addInsert(placeholder)">
                {{ placeholder.name }}
            </VListItem>
            <VListItem v-if="filteredPlaceholders?.length === 0">{{ $t("search.noResults") }}</VListItem>
        </VList>
    </VMenu>
</template>

<script setup lang="ts">
import type { MenuProps } from "vue-text-insert";

const props = defineProps<MenuProps<PlaceholderInsertTileContract>>();

const placeholderStore = usePlaceholderStore();

const { placeholders } = storeToRefs(placeholderStore);

const open = ref(true);

watch(open, () => {
    if (open.value === false) {
        props.menu.value.closeMenu();
    }
});

const addInsert = (placeholder: PlaceholderContract) => {
    props.menu.value.addInsert({
        kind: "Placeholder",
        placeholderId: placeholder.id,
        placeholderName: placeholder.name,
        placeholderVisibility: placeholder.visibility,
        textValue: null
    });
};

const filteredPlaceholders = computed(() => {
    return placeholders.value;
});
</script>
