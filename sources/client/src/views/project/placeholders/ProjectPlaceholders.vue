<template>
    <v-card>
        <v-card-title>
            {{ $t("projectPlaceholdersView.title") }}
            <v-spacer />
            <v-btn-icon @click="inEditMode = !inEditMode" :icon="inEditMode ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <template v-if="inEditMode">
                <ProjectPlaceholdersEdit />
                <ProjectPlaceholderCreate />
            </template>
            <v-chip-group v-else>
                <ProjectPlaceholdersView />
            </v-chip-group>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import ProjectPlaceholdersView from "@/views/project/placeholders/components/ProjectPlaceholdersView.vue";
import ProjectPlaceholdersEdit from "@/views/project/placeholders/components/ProjectPlaceholdersEdit.vue";
import ProjectPlaceholderCreate from "@/views/project/placeholders/components/ProjectPlaceholderCreate.vue";

const inEditMode = ref(false);
</script>
