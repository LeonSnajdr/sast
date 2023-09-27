<template>
    <h1>Project detail</h1>
    <InputText v-model="command"></InputText>
    <Btn @click="executeCommand">{{ $t("test.test") }}</Btn>
    <RouterLink :to="{ name: 'project' }">Close project</RouterLink>
</template>

<script setup lang="ts">
import { ref } from "vue";
import * as commands from "@/bindings";
import { useProjectStore } from "@/stores/projectStore";

const projectStore = useProjectStore();

const command = ref("");

const executeCommand = async () => {
    const test = await commands.runCommand(command.value);

    console.log(test);
};
</script>
