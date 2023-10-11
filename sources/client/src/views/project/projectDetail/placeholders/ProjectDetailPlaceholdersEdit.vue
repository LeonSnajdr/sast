<template>
    <v-card :loading="loading">
        <v-card-title>
            {{ $t("projectDetailPlaceholdersEdit.title") }}
            <v-spacer />
            <v-btn @click="inEditMode = false" icon="mdi-close" size="small" />
        </v-card-title>
        <v-card-text>
            <v-row v-for="placeholder in project.placeholders" :key="placeholder.id">
                <v-col cols="11">
                    <v-text-field :label="placeholder.name" v-model="placeholder.value" @update:modelValue="placeholderChanged(placeholder)"></v-text-field>
                </v-col>
                <v-col cols="1">
                    <v-btn @click="deletePlaceholder(placeholder)" variant="plain" block>
                        <v-icon icon="mdi-delete" />
                    </v-btn>
                </v-col>
            </v-row>
            <v-row>
                <v-col cols="11">
                    <v-text-field v-model="placeholderName" :placeholder="$t('projectDetailPlaceholdersEdit.input.name')"></v-text-field>
                </v-col>
                <v-col cols="1">
                    <v-btn @click="createPlaceholder" variant="plain" block>
                        <v-icon icon="mdi-plus" />
                    </v-btn>
                </v-col>
            </v-row>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import type { CreatePlaceholderContract, Placeholder, UpdatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";
import { remove } from "lodash";

const { inEditMode } = defineModels<{
    inEditMode: boolean;
}>();

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const loading = ref(false);
const placeholderName = ref("");

const createPlaceholder = async () => {
    loading.value = true;

    const createContract: CreatePlaceholderContract = {
        project_id: project.value.id,
        name: placeholderName.value,
        value: ""
    };

    try {
        const createdPlaceholder = await commands.createPlaceholder(createContract);
        project.value.placeholders.push(createdPlaceholder);

        notify.success("projectDetailPlaceholdersEdit.create.success");
    } catch (error) {
        console.error("Could not create placeholder", error);
        notify.error("projectDetailPlaceholdersEdit.create.error");
    } finally {
        placeholderName.value = "";
        loading.value = false;
    }
};

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

const deletePlaceholder = async (placeholder: Placeholder) => {
    loading.value = true;
    try {
        await commands.deletePlaceholder(placeholder.id);
        remove(project.value.placeholders, placeholder);

        notify.success("projectDetailPlaceholdersEdit.delete.success");
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("projectDetailPlaceholdersEdit.delete.error");
    } finally {
        loading.value = false;
    }
};
</script>
