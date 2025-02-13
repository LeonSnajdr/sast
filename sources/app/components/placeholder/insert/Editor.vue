<template>
    <VField class="pa-2 mb-2" variant="outlined" active>
        <VueTextInsertEditor v-model="tiles" :editorOptions="options" />
    </VField>
</template>

<script setup lang="ts">
import { PlaceholderInsertChip, PlaceholderInsertMenu } from "#components";
import type { EditorOptions } from "vue-text-insert";
import { VueTextInsertEditor } from "vue-text-insert";

const tiles = defineModel<PlaceholderInsertTileContract[]>({ required: true });

const options: EditorOptions<PlaceholderInsertTileContract> = {
    textType: "Text",
    typeField: "kind",
    valueField: "textValue",
    insertOptions: {
        Placeholder: {
            trigger: "@",
            insertComponent: PlaceholderInsertChip,
            menuComponent: PlaceholderInsertMenu
        }
    }
};
</script>
