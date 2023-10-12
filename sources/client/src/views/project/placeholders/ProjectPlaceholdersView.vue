<template>
    <v-card>
        <v-card-title>
            {{ $t("projectPlaceholdersView.title") }}
            <v-spacer />

            <v-icon @click="inEditMode = true" icon="mdi-pencil" />
        </v-card-title>
        <v-card-text class="d-flex">
            <v-chip-group>
                <v-chip v-for="placeholder in project.placeholders" :key="placeholder.id">
                    <strong>{{ placeholder.name }}</strong>
                    <span>: {{ placeholder.value }}</span>
                </v-chip>
            </v-chip-group>
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
