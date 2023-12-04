<template>
    <v-row-single>
        <v-form v-model="valid" ref="form">
            <v-row>
                <v-col>
                    <v-text-field
                        v-model="placeholderName"
                        :placeholder="$t('placeholderCreate.input.name')"
                        :rules="[
                            required($t('placeholderCreate.input.name.required')),
                            unique(
                                placeholders.map((p) => p.name),
                                $t('placeholderCreate.input.name.notUnique')
                            )
                        ]"
                        prependIcon="mdi-blur-off"
                    ></v-text-field>
                </v-col>
                <v-col>
                    <v-text-field v-model="placeholderValue" :placeholder="$t('placeholderCreate.input.value')">
                        <template #append>
                            <v-btn-icon @click="createPlaceholder" icon="mdi-plus" />
                        </template>
                    </v-text-field>
                </v-col>
            </v-row>
        </v-form>
    </v-row-single>
</template>

<script setup lang="ts">
import { VForm } from "vuetify/components";
import type { CreatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";
import { required, unique } from "@/rules";
import OrderService from "@/services/OrderService";

const notify = useNotificationStore();
const projectStore = useProjectStore();
const placeholderStore = usePlaceholderStore();

const { selectedProjectId } = storeToRefs(projectStore);
const { placeholders } = storeToRefs(placeholderStore);
const form = ref<VForm>();
const loading = ref(false);
const valid = ref(false);
const placeholderName = ref("");
const placeholderValue = ref("");

const createPlaceholder = async () => {
    await form.value.validate();

    if (!valid.value) return;

    loading.value = true;

    const followingOrderNumber = OrderService.getFollowingOrderNumber(placeholders.value);

    const createContract: CreatePlaceholderContract = {
        project_id: selectedProjectId.value,
        order: followingOrderNumber,
        name: placeholderName.value,
        value: placeholderValue.value
    };

    try {
        await commands.createPlaceholder(createContract);
        await placeholderStore.loadPlaceholderList();

        notify.success("placeholderCreate.create.success");
    } catch (error) {
        console.error("Could not create placeholder", error);
        notify.error("placeholderCreate.create.error");
    } finally {
        form.value.reset();
        loading.value = false;
    }
};
</script>
