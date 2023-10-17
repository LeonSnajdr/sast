<template>
    <v-card>
        <v-card-title>
            {{ $t("projectPlaceholders.title") }}
            <v-spacer />

            <v-btn-icon @click="inEditMode = !inEditMode" :icon="inEditMode ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <template v-if="inEditMode">
                <ProjectPlaceholderEdit
                    v-for="(placeholder, index) in project.placeholders"
                    :key="placeholder.id"
                    v-model:placeholder="project.placeholders[index]"
                />
                <ProjectPlaceholderCreate />
            </template>
            <template v-else>
                <v-chip-group v-if="project.placeholders.length > 0">
                    <ProjectPlaceholderView v-for="placeholder in project.placeholders" :key="placeholder.id" :placeholder="placeholder" />
                </v-chip-group>
                <span v-else>{{ $t("projectPlaceholders.noItems") }}</span>
            </template>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import ProjectPlaceholderView from "@/views/project/placeholders/components/ProjectPlaceholderView.vue";
import ProjectPlaceholderEdit from "@/views/project/placeholders/components/ProjectPlaceholderEdit.vue";
import ProjectPlaceholderCreate from "@/views/project/placeholders/components/ProjectPlaceholderCreate.vue";

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const inEditMode = ref(false);
</script>
