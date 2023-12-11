<template>
    <v-dialog v-model="dialog" activator="parent">
        <v-form v-model="valid" ref="form">
            <v-card>
                <v-card-title>{{ $t("projectCreateDialog.title") }}</v-card-title>
                <v-card-text>
                    <v-text-field
                        v-model="projectName"
                        :label="$t('projectCreateDialog.input.name')"
                        :rules="[
                            required($t('projectCreateDialog.input.name.required')),
                            unique(
                                listProjects.map((lp) => lp.name),
                                $t('projectCreateDialog.input.name.notUnique')
                            )
                        ]"
                    ></v-text-field>
                </v-card-text>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn @click="create" :disabled="!valid">{{ $t("create") }}</v-btn>
                    <v-btn @click="dialog = false">{{ $t("close") }}</v-btn>
                </v-card-actions>
            </v-card>
        </v-form>
    </v-dialog>
</template>

<script setup lang="ts">
import type { CreateProjectContract } from "@/bindings";
import * as commands from "@/bindings";
import { required, unique } from "@/rules";
import { VForm } from "vuetify/components";

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { listProjects } = storeToRefs(projectStore);
const form = ref<VForm>();
const dialog = ref(false);
const valid = ref(false);
const projectName = ref("");

const create = async () => {
    const createContract: CreateProjectContract = {
        name: projectName.value
    };

    try {
        const createdProject = await commands.createProject(createContract);
        listProjects.value.push(createdProject);

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
