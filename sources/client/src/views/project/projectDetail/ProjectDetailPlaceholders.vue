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
import { ref } from "vue";
import ProjectDetailPlaceholder from "./ProjectDetailPlaceholder.vue";
import { useNotificationStore } from "@/stores/notificationStore";

const { project } = defineModels<{
    project: FullProjectContract;
}>();

const notify = useNotificationStore();

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

        notify.success("TODO");
    } catch (error) {
        console.error("Creating placeholder failed", error);
        notify.error("TODO");
    } finally {
        addPlaceholderName.value = "";
        addPlaceholderValue.value = "";
    }
};
</script>
