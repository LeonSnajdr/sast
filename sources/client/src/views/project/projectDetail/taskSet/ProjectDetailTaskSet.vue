<template>
    <v-list-item class="pa-0 pb-2">
        <template #prepend>
            <v-icon v-if="!executing" @click="startTaskSet" icon="mdi-play" color="success"></v-icon>
            <v-progress-circular style="margin-right: 9px" v-else indeterminate></v-progress-circular>
        </template>

        <v-list-item-title>{{ taskSet.name }}</v-list-item-title>
        <v-list-item-subtitle>TODO: Beschreibung hinzufügen</v-list-item-subtitle>

        <template #append>
            <v-icon icon="mdi-cog-outline" />
        </template>
    </v-list-item>
</template>

<script setup lang="ts">
import type { TaskSet } from "@/bindings";
import * as commands from "@/bindings";

const props = defineProps<{
    taskSet: TaskSet;
}>();

const notify = useNotificationStore();

const executing = ref(false);

const startTaskSet = async () => {
    executing.value = true;

    try {
        await commands.startTaskSet(props.taskSet.id);
    } catch (error) {
        console.error("Error while executing taskset", error);
        notify.error("projectDetailTaskSet.execute.error");
    } finally {
        executing.value = false;
    }
};
</script>
