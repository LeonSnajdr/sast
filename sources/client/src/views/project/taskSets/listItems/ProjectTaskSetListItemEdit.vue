<template>
    <v-list-item-single>
        <v-text-field :label="taskSet.name">
            <template #append>
                <v-icon icon="mdi-tune" size="20"> </v-icon>
                <ProjectTaskSetEditDialog v-model:taskSet="taskSet" />
            </template>
        </v-text-field>
    </v-list-item-single>
</template>

<script setup lang="ts">
import type { FullTaskSetContract } from "@/bindings";
import ProjectTaskSetEditDialog from "../ProjectTaskSetEditDialog.vue";

const { taskSet } = defineModels<{
    taskSet: FullTaskSetContract;
}>();
</script>
