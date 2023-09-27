<template>
    <h1>Project detail</h1>
    <InputText v-model="command"></InputText>
    <Button @click="executeCommand">{{ $t("test.test") }}</Button>
</template>

<script setup lang="ts">
import InputText from "primevue/inputtext";
import Button from "primevue/button";

import { ref } from "vue";

import * as commands from "@/bindings";

const command = ref("");

const executeCommand = async () => {
    const test = await commands.runCommand(command.value);

    console.log(test);
};
</script>
