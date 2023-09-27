<template>
    <h1>Project detail</h1>
    <InputText v-model="command"></InputText>
    <Btn @click="executeCommand" :loading="loading">{{ $t("test.test") }}</Btn>
    <RouterLink :to="{ name: 'project' }">Close project</RouterLink>
</template>

<script setup lang="ts">
import { ref } from "vue";
import * as commands from "@/bindings";
import { useProjectStore } from "@/stores/projectStore";

const projectStore = useProjectStore();

const command = ref("");
const loading = ref(false);

const executeCommand = async () => {
    loading.value = true;
    const test = await commands.runCommand(command.value);

    loading.value = false;

    console.log(test);
};
</script>
