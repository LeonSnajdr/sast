<template>
    <v-card>
        <v-card-title>
            {{ $t("placeholders.title") }}
            <v-spacer />

            <v-btn-icon @click="inPlaceholderEdit = !inPlaceholderEdit" :icon="inPlaceholderEdit ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <template v-if="inPlaceholderEdit">
                <placeholderEdit v-for="(placeholder, index) in project.placeholders" :key="placeholder.id" v-model:placeholder="project.placeholders[index]" />
                <placeholderCreate />
            </template>
            <template v-else>
                <v-chip-group v-if="project.placeholders.length > 0">
                    <placeholderView v-for="placeholder in project.placeholders" :key="placeholder.id" :placeholder="placeholder" />
                </v-chip-group>
                <span v-else>{{ $t("placeholders.noItems") }}</span>
            </template>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import placeholderView from "@/views/project/placeholders/PlaceholderView.vue";
import placeholderEdit from "@/views/project/placeholders/PlaceholderEdit.vue";
import placeholderCreate from "@/views/project/placeholders/PlaceholderCreate.vue";

const projectPageStore = useProjectPageStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const { inPlaceholderEdit } = storeToRefs(projectPageStore);
</script>
