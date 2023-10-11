<template>
    <v-dialog v-model="dialog" activator="parent" width="500">
        <v-form v-model="valid" ref="form">
            <v-card>
                <v-card-title>{{ $t("projectCreateDialog.title") }}</v-card-title>
                <v-card-text>
                    <v-text-field
                        v-model="projectName"
                        :label="$t('projectCreateDialog.input.name')"
                        :rules="[required($t('projectCreateDialog.input.name.required'))]"
                    ></v-text-field>
                </v-card-text>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn @click="create" :disabled="!valid" color="primary">{{ $t("create") }}</v-btn>
                    <v-btn @click="dialog = false">{{ $t("close") }}</v-btn>
                </v-card-actions>
            </v-card>
        </v-form>
    </v-dialog>
</template>

<script setup lang="ts">
import { VForm } from "vuetify/components";
import type { CreateProjectContract } from "@/bindings";
import * as commands from "@/bindings";
import { required } from "@/rules";

const notify = useNotificationStore();
const projectStore = useProjectStore();

const form = ref<VForm>();
const dialog = ref(false);
const valid = ref(false);
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
        form.value.reset();
        dialog.value = false;
    }
};
</script>
