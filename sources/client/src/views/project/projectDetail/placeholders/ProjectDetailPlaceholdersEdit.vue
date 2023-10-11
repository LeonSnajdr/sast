<template>
    <v-card :loading="loading">
        <v-card-title>
            {{ $t("projectDetailPlaceholdersEdit.title") }}
            <v-spacer />
            <v-btn @click="inEditMode = false" variant="plain">
                <v-icon icon="mdi-close" />
            </v-btn>
        </v-card-title>
        <v-card-text>
            <div v-for="placeholder in project.placeholders" :key="placeholder.id" class="d-flex mb-4">
                <v-text-field :label="placeholder.name" v-model="placeholder.value" @update:modelValue="placeholderChanged(placeholder)"></v-text-field>
                <v-btn @click="deletePlaceholder(placeholder)" variant="plain">
                    <v-icon icon="mdi-delete" />
                </v-btn>
            </div>

            <v-form v-model="valid" class="d-flex">
                <v-row>
                    <v-col>
                        <v-text-field
                            v-model="placeholderName"
                            :placeholder="$t('projectDetailPlaceholdersEdit.input.name')"
                            :rules="[required($t('projectDetailPlaceholdersEdit.input.name.required'))]"
                        ></v-text-field>
                    </v-col>
                    <v-col>
                        <v-text-field
                            v-model="placeholderValue"
                            :placeholder="$t('projectDetailPlaceholdersEdit.input.value')"
                            :rules="[required($t('projectDetailPlaceholdersEdit.input.value.required'))]"
                        ></v-text-field>
                    </v-col>
                </v-row>
                <v-btn @click="createPlaceholder" :disabled="!valid" variant="plain">
                    <v-icon icon="mdi-plus" />
                </v-btn>
            </v-form>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import type { CreatePlaceholderContract, Placeholder, UpdatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";
import { required } from "@/rules";
import { remove } from "lodash";

const { inEditMode } = defineModels<{
    inEditMode: boolean;
}>();

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const loading = ref(false);
const valid = ref(false);
const placeholderName = ref("");
const placeholderValue = ref("");

const createPlaceholder = async () => {
    loading.value = true;

    const createContract: CreatePlaceholderContract = {
        project_id: project.value.id,
        name: placeholderName.value,
        value: placeholderValue.value
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
        placeholderValue.value = "";

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
