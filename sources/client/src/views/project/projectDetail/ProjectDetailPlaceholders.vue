<template>
    <template v-for="(placeholder, index) in project.placeholders" :key="placeholder.id">
        <ProjectDetailPlaceholder v-model:placeholder="project.placeholders[index]" />
    </template>

    <div>
        <InputText v-model="addPlaceholderName" :placeholder="$t('projectDetailPlaceholders.new.input')" size="small"></InputText>
        <InputText v-model="addPlaceholderValue" size="small" />
        <Btn @click="addPlaceholder" :label="$t('placeholders.new.button')"></Btn>
    </div>
</template>

<script setup lang="ts">
import type { CreatePlaceholderContract, FullProjectContract } from "@/bindings";
import { createPlaceholder } from "@/bindings";
import { useToast } from "primevue/usetoast";
import { ref } from "vue";
import ProjectDetailPlaceholder from "./ProjectDetailPlaceholder.vue";

const { project } = defineModels<{
    project: FullProjectContract;
}>();

const toast = useToast();

const addPlaceholderName = ref("");
const addPlaceholderValue = ref("");

const addPlaceholder = async () => {
    const createContract: CreatePlaceholderContract = {
        project_id: project.value.id,
        name: addPlaceholderName.value,
        value: addPlaceholderValue.value
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
        addPlaceholderValue.value = "";
    }
};
</script>
