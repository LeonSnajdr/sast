<template>
    <v-card>
        <v-card-title>
            {{ $t("projectPlaceholdersView.title") }}
            <v-spacer />

            <v-icon @click="inEditMode = true" icon="mdi-pencil" />
        </v-card-title>
        <v-card-text class="d-flex">
            <div v-for="placeholder in project.placeholders" :key="placeholder.id">
                <v-chip class="mr-1">{{ placeholder.value }}</v-chip>
            </div>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
const { inEditMode } = defineModels<{
    inEditMode: boolean;
}>();

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
</script>
