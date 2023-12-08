<template>
    <v-row-single>
        <v-form v-model="valid" ref="form" class="d-flex">
            <v-row>
                <v-col>
                    <v-text-field
                        v-model="taskSetName"
                        :placeholder="$t('taskSetCreate.input.name')"
                        :rules="[
                            required($t('taskSetCreate.input.name.required')),
                            unique(
                                taskSets.map((ts) => ts.name),
                                $t('taskSetCreate.input.name.notUnique')
                            )
                        ]"
                        prependIcon="mdi-blur-off"
                    ></v-text-field>
                </v-col>
                <v-col>
                    <v-text-field v-model="taskSetDescription" :placeholder="$t('taskSetCreate.input.description')" @keydown.enter="createTaskSet">
                        <template #append>
                            <v-btn-icon @click="createTaskSet" icon="mdi-plus" />
                        </template>
                    </v-text-field>
                </v-col>
            </v-row>
        </v-form>
    </v-row-single>
</template>

<script setup lang="ts">
import type { CreateTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";
import { required, unique } from "@/rules";
import { VForm } from "vuetify/components";
import OrderService from "@/services/OrderService";

const notify = useNotificationStore();
const projectStore = useProjectStore();
const taskSetStore = useTaskSetStore();

const { selectedProjectId } = storeToRefs(projectStore);
const { taskSets } = storeToRefs(taskSetStore);

const form = ref<VForm>();
const valid = ref(false);
const taskSetName = ref("");
const taskSetDescription = ref("");

const createTaskSet = async () => {
    await form.value.validate();

    if (!valid.value) return;

    const followingOrderNumber = OrderService.getFollowingOrderNumber(taskSets.value);

    const createContract: CreateTaskSetContract = {
        project_id: selectedProjectId.value,
        order: followingOrderNumber,
        description: taskSetDescription.value,
        name: taskSetName.value
    };

    try {
        await commands.createTaskSet(createContract);
        await taskSetStore.loadTaskSetList();

        notify.success("taskSetCreate.create.success");
    } catch (error) {
        console.error("Could not create placeholder", error);
        notify.error("taskSetCreate.create.error");
    } finally {
        form.value.reset();
    }
};
</script>
