<template>
    <v-dialog v-model="dialog" activator="parent" width="auto">
        <v-card>
            <v-card-title>{{ $t("projectCreateDialog.title") }}</v-card-title>
            <v-card-text>
                <v-text-field v-model="projectName" :label="$t('projectCreateDialog.input.name')"></v-text-field>
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn @click="create" color="primary">{{ $t("create") }}</v-btn>
                <v-btn @click="dialog = false">{{ $t("close") }}</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import type { CreateProjectContract } from "@/bindings";
import * as commands from "@/bindings";

const notify = useNotificationStore();
const projectStore = useProjectStore();

const dialog = ref(false);
const projectName = ref("");

const create = async () => {
    const createContract: CreateProjectContract = {
        name: projectName.value
    };

    try {
        await commands.createProject(createContract);
        await projectStore.loadListProjects();

        notify.success("projectCreateDialog.create.success");
    } catch (error) {
        console.error("Could not create project", error);
        notify.error("projectCreateDialog.create.error");
    } finally {
        projectName.value = "";
        dialog.value = false;
    }
};
</script>
