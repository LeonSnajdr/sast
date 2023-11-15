<template>
    <v-row-single>
        <v-form v-model="valid" ref="form" class="d-flex">
            <v-row>
                <v-col>
                    <v-text-field
                        v-model="taskSetName"
                        :placeholder="$t('projectTaskSetCreate.input.name')"
                        :rules="[
                            required($t('projectTaskSetCreate.input.name.required')),
                            unique(
                                project.task_sets.map((ts) => ts.name),
                                $t('projectTaskSetCreate.input.name.notUnique')
                            )
                        ]"
                    ></v-text-field>
                </v-col>
                <v-col>
                    <v-text-field v-model="taskSetDescription" :placeholder="$t('projectTaskSetCreate.input.description')">
                        <template #append>
                            <v-btn-icon @click="createTaskSet" icon="mdi-plus" />
                        </template>
                    </v-text-field>
                </v-col>
            </v-row>
        </v-form>
    </v-row-single>
</template>

<script setup lang="ts">
import type { CreateTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";
import { required, unique } from "@/rules";
import { VForm } from "vuetify/components";
import { max } from "lodash";

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const form = ref<VForm>();
const loading = ref(false);
const valid = ref(false);
const taskSetName = ref("");
const taskSetDescription = ref("");

const createTaskSet = async () => {
    await form.value.validate();

    if (!valid.value) return;

    loading.value = true;

    const highestOrderNumber = max(project.value.task_sets.map((ts) => ts.order)) ?? 0;

    const createContract: CreateTaskSetContract = {
        project_id: project.value.id,
        order: highestOrderNumber + 1,
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
