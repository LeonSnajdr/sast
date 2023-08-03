<template>
    <div class="card flex justify-content-center">
        <Button label="Check" icon="pi pi-check" @click="click" />
        <Button label="Create" @click="create" />
        <h2>{{ result }}</h2>
    </div>
</template>

<script setup lang="ts">
import Button from "primevue/button";
import * as commands from "./bindings";
import { ref } from "vue";

const result = ref<string>();

async function click(): Promise<void> {
    // result.value = await commands.greet({ name: "Keck" });
}

async function create(): Promise<void> {
    await commands.createProject({ name: "keckw" });
}
</script>
