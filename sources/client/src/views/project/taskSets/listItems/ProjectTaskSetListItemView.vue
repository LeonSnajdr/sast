<template>
    <v-list-item>
        <template #prepend>
            <v-icon v-if="!executing" @click="startTaskSet" icon="mdi-play" color="success"></v-icon>
            <v-progress-circular style="margin-right: 9px" v-else indeterminate></v-progress-circular>
        </template>

        <v-list-item-title>{{ taskSet.name }}</v-list-item-title>
        <v-list-item-subtitle>{{ taskSet.description }}</v-list-item-subtitle>
    </v-list-item>
</template>

<script setup lang="ts">
import type { FullTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";

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
        notify.error("projectTaskSetListItemView.execute.error");
    } finally {
        executing.value = false;
    }
};
</script>
