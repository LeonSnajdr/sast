<template>
    <v-row-single>
        <v-text-field v-model="internalPlaceholder.value" :label="internalPlaceholder.name" @update:modelValue="placeholderChanged" prependIcon="mdi-drag">
            <template #append>
                <v-btn-icon icon="mdi-delete">
                    <ConfirmationDialog
                        :message="$t('placeholderEdit.delete.confirmation', { placeholderName: internalPlaceholder.name })"
                        @confirmAction="deletePlaceholder"
                    />
                </v-btn-icon>
            </template>
        </v-text-field>
    </v-row-single>
</template>

<script setup lang="ts">
import { debounce } from "lodash";
import type { Placeholder, UpdatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";

const props = defineProps<{
    placeholder: Placeholder;
}>();

const notify = useNotificationStore();
const placeholderStore = usePlaceholderStore();

const internalPlaceholder = ref<Placeholder>();

onBeforeMount(() => {
    internalPlaceholder.value = Object.create(props.placeholder);
});

const placeholderChanged = debounce(async () => {
    console.log("update");

    const updateContract: UpdatePlaceholderContract = {
        id: internalPlaceholder.value.id,
        order: internalPlaceholder.value.order,
        value: internalPlaceholder.value.value
    };

    try {
        await commands.updatePlaceholder(updateContract);
        await placeholderStore.loadPlaceholderList();
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("placeholderEdit.update.error");
    }
}, 500);

const deletePlaceholder = async () => {
    try {
        await commands.deletePlaceholder(internalPlaceholder.value.id);
        await placeholderStore.loadPlaceholderList();

        notify.success("placeholderEdit.delete.success");
    } catch (error) {
        console.error("Updating placeholder failed", error);
        notify.error("placeholderEdit.delete.error");
    }
};
</script>
