<template>
    <InputText v-model="placeholder.value" @update:modelValue="valueUpdated" size="small" :placeholder="placeholder.name"></InputText>
</template>

<script setup lang="ts">
import type { Placeholder, UpdatePlaceholderContract } from "@/bindings";
import * as commands from "@/bindings";

const { placeholder } = defineModels<{
    placeholder: Placeholder;
}>();

const valueUpdated = async () => {
    const updateContract: UpdatePlaceholderContract = {
        id: placeholder.value.id,
        value: placeholder.value.value
    };

    await commands.updatePlaceholder(updateContract);
};
</script>
