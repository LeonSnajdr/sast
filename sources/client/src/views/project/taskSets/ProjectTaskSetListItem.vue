<template>
    <v-list-item class="pa-0 pb-2">
        <template #prepend>
            <v-icon v-if="!executing" @click="startTaskSet" icon="mdi-play" color="success"></v-icon>
            <v-progress-circular style="margin-right: 9px" v-else indeterminate></v-progress-circular>
        </template>

        <v-list-item-title>{{ taskSet.name }}</v-list-item-title>
        <v-list-item-subtitle>TODO: Beschreibung hinzufügen </v-list-item-subtitle>

        <template #append>
            <button>
                <v-icon icon="mdi-tune" size="20"> </v-icon>
                <ProjectTaskSetEditDialog v-model:taskSet="taskSet" />
            </button>
        </template>
    </v-list-item>
</template>

<script setup lang="ts">
import type { FullTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";
import ProjectTaskSetEditDialog from "./ProjectTaskSetEditDialog.vue";

const { taskSet } = defineModels<{
    taskSet: FullTaskSetContract;
}>();

const notify = useNotificationStore();

const executing = ref(false);

const startTaskSet = async () => {
    executing.value = true;

    try {
        await commands.startTaskSet(taskSet.value.id);
    } catch (error) {
        console.error("Error while executing taskset", error);
        notify.error("projectTaskSetListItem.execute.error");
    } finally {
        executing.value = false;
    }
};
</script>
