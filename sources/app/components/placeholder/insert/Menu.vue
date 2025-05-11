<template>
    <VMenu v-model="open" :target="menu.value.position">
        <VList>
            <VListItem v-for="placeholderResult in placeholderResults" :key="placeholderResult.item.id" @click="addInsert(placeholderResult.item)">
                <template #prepend>
                    <PlaceholderIcon :visibility="placeholderResult.item.visibility" size="small" />
                </template>
                {{ placeholderResult.item.name }}
            </VListItem>
            <VListItem v-if="placeholderResults?.length === 0">{{ $t("search.noResults") }}</VListItem>
        </VList>
    </VMenu>
</template>

<script setup lang="ts">
import { useFuse } from "@vueuse/integrations/useFuse";
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

const { results: placeholderResults } = useFuse(() => props.menu.value.query, placeholders, {
    fuseOptions: {
        keys: ["name"],
        isCaseSensitive: false
    },
    matchAllWhenSearchEmpty: true
});
</script>
