<template>
    <VMenu v-model="open" :target="menu.value.position">
        <VList>
            <VListItem v-for="placeholder in placeholders" :key="placeholder.id" @click="addInsert(placeholder)">
                {{ placeholder.name }}
            </VListItem>
        </VList>
    </VMenu>
</template>

<script setup lang="ts">
import type { MenuProps } from "vue-text-insert";

const props = defineProps<MenuProps<PlaceholderInsertTileContract>>();

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
        textValue: null
    });
};

const placeholders = asyncComputed(async () => {
    const placeholderResult = await commands.placeholderGetAllGlobal();

    if (placeholderResult.status === "error") {
        return;
    }

    return placeholderResult.data;
}, []);
</script>
