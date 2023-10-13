<template>
    <v-row-single>
        <v-form v-model="valid" ref="form" class="d-flex">
            <v-row>
                <v-col>
                    <v-text-field
                        v-model="taskSetName"
                        :placeholder="$t('projectTaskSetCreate.input.name')"
                        :rules="[required($t('projectTaskSetCreate.input.name.required'))]"
                    ></v-text-field>
                </v-col>
                <v-col>
                    <v-text-field
                        v-model="taskSetDescription"
                        :placeholder="$t('projectTaskSetCreate.input.description')"
                        @click:append="createTaskSet"
                        appendIcon="mdi-plus"
                    />
                </v-col>
            </v-row>
        </v-form>
    </v-row-single>
</template>

<script setup lang="ts">
import { VForm } from "vuetify/components";
import type { CreateTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";

import { required } from "@/rules";

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const form = ref<VForm>();
const loading = ref(false);
const valid = ref(false);
const taskSetName = ref("");
const taskSetDescription = ref("");

const createTaskSet = async () => {
    if (!valid.value) return;

    loading.value = true;

    const createContract: CreateTaskSetContract = {
        project_id: project.value.id,
        description: taskSetDescription.value,
        name: taskSetName.value
    };

    try {
        const createdTaskSet = await commands.createTaskSet(createContract);
        project.value.task_sets.push(createdTaskSet);

        notify.success("projectTaskSetCreate.create.success");
    } catch (error) {
        console.error("Could not create placeholder", error);
        notify.error("projectTaskSetCreate.create.error");
    } finally {
        form.value.reset();
        loading.value = false;
    }
};
</script>
