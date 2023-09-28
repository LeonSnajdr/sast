<template>
    <template v-for="placeholder in project.placeholders" :key="placeholder.id">
        <InputText v-model="placeholder.value" size="small" :placeholder="placeholder.name"></InputText>
    </template>

    <div>
        <InputText v-model="addPlaceholderName" :placeholder="$t('projectDetailPlaceholders.new.input')" size="small"></InputText>
        <Dropdown v-model="addPlaceholderVariety" :options="Object.values(PlaceholderVariety)" size="small" />
        <Btn @click="addPlaceholder" :label="$t('placeholders.new.button')"></Btn>
    </div>
</template>

<script setup lang="ts">
import type { CreatePlaceholderContract, FullProjectContract } from "@/bindings";
import { createPlaceholder } from "@/bindings";
import { useToast } from "primevue/usetoast";
import { ref } from "vue";

const { project } = defineModels<{
    project: FullProjectContract;
}>();

const toast = useToast();

const addPlaceholderName = ref("");
const addPlaceholderVariety = ref("");

const addPlaceholder = async () => {
    const createContract: CreatePlaceholderContract = {
        project_id: project.value.id,
        name: addPlaceholderName.value,
        variety: addPlaceholderVariety.value
    };

    try {
        const newContract = await createPlaceholder(createContract);

        project.value.placeholders.push(newContract);

        toast.add({ severity: "success", detail: "Placeholder created", group: "br", life: 3000 });
    } catch (error) {
        console.error("Creating placeholder failed", error);
        toast.add({ severity: "error", detail: "Placeholder creation failed", group: "br", life: 3000 });
    } finally {
        addPlaceholderName.value = "";
        addPlaceholderVariety.value = "";
    }
};

enum PlaceholderVariety {
    Input = "Input",
    Select = "Select"
}
</script>
