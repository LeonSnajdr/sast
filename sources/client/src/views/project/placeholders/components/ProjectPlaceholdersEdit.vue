<template>
    <v-row-single v-for="placeholder in project.placeholders" :key="placeholder">
        <v-text-field v-model="placeholder.value" :label="placeholder.name" @update:modelValue="placeholderChanged(placeholder)">
            <template #append>
                <v-btn-icon @click="deletePlaceholder(placeholder)" icon="mdi-delete" />
            </template>
        </v-text-field>
    </v-row-single>
</template>

<script setup lang="ts">
import type { Placeholder, UpdatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";
import { remove } from "lodash";

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const loading = ref(false);

const placeholderChanged = async (placeholder: Placeholder) => {
    loading.value = true;

    const updateContract: UpdatePlaceholderContract = {
        id: placeholder.id,
        value: placeholder.value
    };

    try {
        await commands.updatePlaceholder(updateContract);
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("projectPlaceholdersEdit.update.error");
    } finally {
        loading.value = false;
    }
};

const deletePlaceholder = async (placeholder: Placeholder) => {
    loading.value = true;
    try {
        await commands.deletePlaceholder(placeholder.id);
        remove(project.value.placeholders, placeholder);

        notify.success("projectPlaceholdersEdit.delete.success");
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("projectPlaceholdersEdit.delete.error");
    } finally {
        loading.value = false;
    }
};
</script>
