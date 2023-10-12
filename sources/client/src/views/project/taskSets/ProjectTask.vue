<template>
    <v-row>
        <v-col :cols="expanded ? 5 : 12">
            <v-text-field v-model="task.command" prependInnerIcon="mdi-apple-keyboard-command">
                <template v-if="!expanded" #append>
                    <v-icon @click="expanded = !expanded" icon="mdi-cog" />
                </template>
            </v-text-field>
        </v-col>
        <template v-if="expanded">
            <v-col cols="5">
                <v-text-field prependInnerIcon="mdi-folder"></v-text-field>
            </v-col>
            <v-col cols="2">
                <v-text-field prependInnerIcon="mdi-clock">
                    <template #append>
                        <v-icon icon="mdi-delete" color="error" />
                        <v-icon @click="expanded = !expanded" icon="mdi-cog" class="ml-1" />
                    </template>
                </v-text-field>
            </v-col>
        </template>
    </v-row>
</template>

<script setup lang="ts">
import type { Task } from "@/bindings";

const expanded = ref(false);

const { task } = defineModels<{
    task: Task;
}>();
</script>
