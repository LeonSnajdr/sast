<template>
    <v-row-single>
        <v-form v-model="valid" ref="form" class="d-flex">
            <v-row>
                <v-col>
                    <v-text-field
                        v-model="placeholderName"
                        :placeholder="$t('projectPlaceholderCreate.input.name')"
                        :rules="[required($t('projectPlaceholderCreate.input.name.required'))]"
                    ></v-text-field>
                </v-col>
                <v-col>
                    <v-text-field v-model="placeholderValue" :placeholder="$t('projectPlaceholderCreate.input.value')">
                        <template #append>
                            <v-btn-icon @click="createPlaceholder" icon="mdi-plus" />
                        </template>
                    </v-text-field>
                </v-col>
            </v-row>
        </v-form>
    </v-row-single>
</template>

<script setup lang="ts">
import { VForm } from "vuetify/components";
import type { CreatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";
import { required } from "@/rules";

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const form = ref<VForm>();
const loading = ref(false);
const valid = ref(false);
const placeholderName = ref("");
const placeholderValue = ref("");

const createPlaceholder = async () => {
    if (!valid.value) return;

    loading.value = true;

    const createContract: CreatePlaceholderContract = {
        project_id: project.value.id,
        name: placeholderName.value,
        value: placeholderValue.value
    };

    try {
        const createdPlaceholder = await commands.createPlaceholder(createContract);
        project.value.placeholders.push(createdPlaceholder);

        notify.success("projectPlaceholderCreate.create.success");
    } catch (error) {
        console.error("Could not create placeholder", error);
        notify.error("projectPlaceholderCreate.create.error");
    } finally {
        form.value.reset();
        loading.value = false;
    }
};
</script>
