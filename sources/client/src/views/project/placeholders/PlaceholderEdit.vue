<template>
    <v-row-single>
        <v-text-field v-model="internalPlaceholder.value" :label="internalPlaceholder.name" @update:modelValue="placeholderChanged">
            <template #prepend>
                <v-icon icon="mdi-drag"></v-icon>
            </template>

            <template #append>
                <v-btn-icon @click="deletePlaceholder" icon="mdi-delete" />
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

const placeholderChanged = async () => {
    const updateContract: UpdatePlaceholderContract = {
        id: internalPlaceholder.value.id,
        order: internalPlaceholder.value.order,
        value: internalPlaceholder.value.value
    };

    try {
        await commands.updatePlaceholder(updateContract);
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("placeholderEdit.update.error");
    }
};

const deletePlaceholder = async () => {
    try {
        await commands.deletePlaceholder(internalPlaceholder.value.id);
        await placeholderStore.loadPlaceholderList();

        notify.success("placeholderEdit.delete.success");
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("placeholderEdit.delete.error");
    }
};
</script>
