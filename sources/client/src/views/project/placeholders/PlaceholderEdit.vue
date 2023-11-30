<template>
    <v-row-single>
        <v-text-field v-model="internalPlaceholder.value" :label="internalPlaceholder.name" @update:modelValue="placeholderChanged(placeholder)">
            <template #append>
                <v-btn-icon @click="deletePlaceholder(placeholder)" icon="mdi-delete" />
            </template>
        </v-text-field>
    </v-row-single>
</template>

<script setup lang="ts">
import type { Placeholder, UpdatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";

const props = defineProps<{
    placeholder: Placeholder;
}>();

const notify = useNotificationStore();
const placeholderStore = usePlaceholderStore();

const internalPlaceholder = ref<Placeholder>();

onBeforeMount(() => {
    internalPlaceholder.value = Object.create(props.placeholder);
});

const placeholderChanged = async (placeholder: Placeholder) => {
    const updateContract: UpdatePlaceholderContract = {
        id: placeholder.id,
        value: placeholder.value
    };

    try {
        await commands.updatePlaceholder(updateContract);
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("placeholderEdit.update.error");
    }
};

const deletePlaceholder = async (placeholder: Placeholder) => {
    try {
        await commands.deletePlaceholder(placeholder.id);
        await placeholderStore.loadPlaceholderList();

        notify.success("placeholderEdit.delete.success");
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("placeholderEdit.delete.error");
    }
};
</script>
