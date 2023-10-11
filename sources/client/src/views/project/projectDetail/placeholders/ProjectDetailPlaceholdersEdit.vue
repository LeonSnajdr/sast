<template>
    <v-card :loading="loading">
        <v-card-title>
            {{ $t("projectDetailPlaceholdersEdit.title") }}
            <v-spacer />

            <v-btn icon="mdi-close" size="small" @click="inEditMode = false" />
        </v-card-title>
        <v-card-text>
            <div v-for="placeholder in project.placeholders" :key="placeholder.id">
                <v-text-field
                    :label="placeholder.name"
                    v-model="placeholder.value"
                    @update:modelValue="placeholderChanged(placeholder)"
                    class="mb-2"
                ></v-text-field>
            </div>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import type { Placeholder, UpdatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";

const { inEditMode } = defineModels<{
    inEditMode: boolean;
}>();

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
        notify.error("projectDetailPlaceholdersEdit.update.error");
    } finally {
        loading.value = false;
    }
};
</script>
