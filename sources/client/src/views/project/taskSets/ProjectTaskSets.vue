<template>
    <v-card>
        <v-card-title>
            {{ $t("projectTaskSets.title") }}
            <v-spacer />
            <v-icon @click="inCreateMode = !inCreateMode" :icon="inCreateMode ? 'mdi-close' : 'mdi-plus'" />
        </v-card-title>
        <v-card-text>
            <v-list>
                <template v-for="(taskSet, index) in project.task_sets" :key="taskSet.id">
                    <ProjectTaskSetListItem v-model:taskSet="project.task_sets[index]" />
                </template>
            </v-list>

            <v-form v-if="inCreateMode" v-model="valid" ref="form" class="d-flex">
                <v-row>
                    <v-col>
                        <v-text-field
                            v-model="taskSetName"
                            :placeholder="$t('projectTaskSets.input.name')"
                            :rules="[required($t('projectTaskSets.input.name.required'))]"
                        ></v-text-field>
                    </v-col>
                    <v-col>
                        <v-text-field
                            v-model="taskSetDescription"
                            :placeholder="$t('projectTaskSets.input.description')"
                            @click:append="createTaskSet"
                            appendIcon="mdi-plus"
                        />
                    </v-col>
                </v-row>
            </v-form>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import { VForm } from "vuetify/components";
import ProjectTaskSetListItem from "./ProjectTaskSetListItem.vue";
import type { CreateTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";
import { required } from "@/rules";

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const form = ref<VForm>();
const inCreateMode = ref(false);
const loading = ref(false);
const valid = ref(false);
const taskSetName = ref("");
const taskSetDescription = ref("");

const createTaskSet = async () => {
    if (!valid.value) return;

    loading.value = true;

    const createContract: CreateTaskSetContract = {
        project_id: project.value.id,
        name: taskSetName.value
    };

    try {
        const createdTaskSet = await commands.createTaskSet(createContract);
        project.value.task_sets.push(createdTaskSet);

        notify.success("projectTaskSets.create.success");
    } catch (error) {
        console.error("Could not create placeholder", error);
        notify.error("projectTaskSets.create.error");
    } finally {
        form.value.reset();
        loading.value = false;
    }
};
</script>
