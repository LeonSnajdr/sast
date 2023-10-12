<template>
    <v-dialog v-model="dialog" activator="parent" width="900">
        <v-form v-model="valid" ref="form">
            <v-card>
                <v-card-title>{{ taskSet.name }}</v-card-title>
                <v-card-text>
                    <v-row-single v-for="(task, index) in taskSet.tasks" :key="task.id">
                        <ProjectTask v-model:task="taskSet.tasks[index]" />
                    </v-row-single>
                    <v-row-single>
                        <v-text-field appendIcon="mdi-plus"></v-text-field>
                    </v-row-single>
                </v-card-text>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn @click="dialog = false">{{ $t("close") }}</v-btn>
                </v-card-actions>
            </v-card>
        </v-form>
    </v-dialog>
</template>

<script setup lang="ts">
import { VForm } from "vuetify/components";
import ProjectTask from "./ProjectTask.vue";
import type { FullTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";
import { required } from "@/rules";

const { taskSet } = defineModels<{
    taskSet: FullTaskSetContract;
}>();

const notify = useNotificationStore();

const form = ref<VForm>();
const dialog = ref(false);
const valid = ref(false);

const update = async () => {
    //
};
</script>
