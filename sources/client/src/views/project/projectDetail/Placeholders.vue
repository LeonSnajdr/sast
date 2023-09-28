<template>
    <template v-for="placeholder in project.placeholders" :key="placeholder.id">
        <InputText v-model="placeholder.value" size="small" :placeholder="placeholder.name"></InputText>
    </template>

    <div>
        <InputText v-model="newPlaceholderName" :placeholder="$t('placeholders.new')" size="small"></InputText>
        <Dropdown v-model="newPlaceholderVariety" :options="Object.values(PlaceholderVariety)" size="small" />
        <Btn @click="createNewPlaceholder" :label="$t('placeholders.new.button')"></Btn>
    </div>
</template>

<script setup lang="ts">
import type { CreatePlaceholderContract, FullProjectContract } from "@/bindings";
import { createPlaceholder } from "@/bindings";
import { useProjectStore } from "@/stores/projectStore";
import { ref } from "vue";

const props = defineProps<{
    project: FullProjectContract;
}>();

const newPlaceholderName = ref("");
const newPlaceholderVariety = ref("");

const createNewPlaceholder = async () => {
    const createContract: CreatePlaceholderContract = {
        project_id: props.project.id,
        name: newPlaceholderName.value,
        variety: newPlaceholderVariety.value
    };

    await createPlaceholder(createContract);

    // props.project.placeholders.push(newContract);
};

enum PlaceholderVariety {
    Input = "Input",
    Select = "Select"
}
</script>
