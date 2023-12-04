<template>
    <v-list-item>
        <template #prepend>
            <v-btn-icon @click="startTaskSet(taskSet.id)" :loading="isTaskSetRunning(taskSet.id)" icon="mdi-play" color="success" class="mr-2" />
        </template>

        <v-list-item-title>{{ taskSet.name }}</v-list-item-title>
        <v-list-item-subtitle>{{ taskSet.description }}</v-list-item-subtitle>
    </v-list-item>
</template>

<script setup lang="ts">
import type { TaskSet } from "@/bindings";

defineProps<{
    taskSet: TaskSet;
}>();

const { startTaskSet, isTaskSetRunning } = useTaskSetStore();
</script>
