<template>
    <v-card>
        <v-card-title>
            {{ $t("placeholders.title") }}
            <v-spacer />

            <v-btn-icon @click="inPlaceholderEdit = !inPlaceholderEdit" :icon="inPlaceholderEdit ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <template v-if="inPlaceholderEdit">
                <draggable v-model="placeholders" @end="placeholderOrderChanged" itemKey="id">
                    <template #item="{ element: placeholder }">
                        <PlaceholderEdit :placeholder="placeholder" />
                    </template>
                </draggable>

                <PlaceholderCreate />
            </template>
            <template v-else>
                <v-chip-group v-if="placeholders.length > 0">
                    <PlaceholderView v-for="placeholder in placeholders" :key="placeholder.id" :placeholder="placeholder" />
                </v-chip-group>
                <span v-else>{{ $t("placeholders.noItems") }}</span>
            </template>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import PlaceholderView from "@/views/project/placeholders/PlaceholderView.vue";
import PlaceholderEdit from "@/views/project/placeholders/PlaceholderEdit.vue";
import PlaceholderCreate from "@/views/project/placeholders/PlaceholderCreate.vue";
import draggable from "vuedraggable";
import * as commands from "@/bindings";
import type { UpdatePlaceholderContract } from "@/bindings";
import OrderService from "@/services/OrderService";

const projectStore = useProjectStore();
const placeholderStore = usePlaceholderStore();

const { selectedProjectId } = storeToRefs(projectStore);
const { inPlaceholderEdit, placeholders } = storeToRefs(placeholderStore);

watch(
    selectedProjectId,
    async () => {
        await placeholderStore.loadPlaceholderList();
    },
    { immediate: true }
);

const placeholderOrderChanged = async () => {
    const placeholderUpdateContracts = OrderService.getItemListWithUpdatedOrders<UpdatePlaceholderContract>(placeholders.value);

    console.log(placeholderUpdateContracts);

    await commands.updatePlaceholders(placeholderUpdateContracts);
    await placeholderStore.loadPlaceholderList();
};
</script>
