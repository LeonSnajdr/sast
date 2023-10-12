<template>
    <v-card>
        <v-card-title>
            {{ $t("projectTaskSetsEdit.title") }}
            <v-spacer />
            <v-icon @click="inEditMode = false" icon="mdi-close" />
        </v-card-title>
        <v-card-text>
            <v-list>
                <template v-for="taskSet in project.task_sets" :key="taskSet.id">
                    <ProjectTaskSetListItem :taskSet="taskSet" />
                </template>
            </v-list>

            <v-form v-model="valid" ref="form" class="d-flex">
                <v-row>
                    <v-col>
                        <v-text-field
                            v-model="taskSetName"
                            :placeholder="$t('projectTaskSetsEdit.input.name')"
                            :rules="[required($t('projectTaskSetsEdit.input.name.required'))]"
                        ></v-text-field>
                    </v-col>
                    <v-col>
                        <v-text-field
                            v-model="taskSetDescription"
                            :placeholder="$t('projectTaskSetsEdit.input.description')"
                            :rules="[required($t('projectTaskSetsEdit.input.description.required'))]"
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
import { remove } from "lodash";

const { inEditMode } = defineModels<{
    inEditMode: boolean;
}>();

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
        name: taskSetName.value
    };

    try {
        const createdTaskSet = await commands.createTaskSet(createContract);
        project.value.task_sets.push(createdTaskSet);

        notify.success("projectTaskSetsEdit.create.success");
    } catch (error) {
        console.error("Could not create placeholder", error);
        notify.error("projectTaskSetsEdit.create.error");
    } finally {
        form.value.reset();
        loading.value = false;
    }
};
</script>
