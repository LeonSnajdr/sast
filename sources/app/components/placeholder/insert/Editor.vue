<template>
    <VField :id class="pa-2 mb-2" variant="outlined" active>
        <input v-show="false" :id="id" />
        <template #label>
            <span>{{ label }}</span>
            <VIcon class="ml-1" icon="mdi-label" size="12" />
        </template>
        <VueTextInsertEditor v-model="tiles" :editorOptions="options" />
    </VField>
</template>

<script setup lang="ts">
import { PlaceholderInsertChip, PlaceholderInsertMenu } from "#components";
import type { EditorOptions } from "vue-text-insert";
import { VueTextInsertEditor } from "vue-text-insert";

defineProps<{
    label: string;
}>();

const tiles = defineModel<PlaceholderInsertTileContract[]>({ required: true });

const id = crypto.randomUUID();

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
