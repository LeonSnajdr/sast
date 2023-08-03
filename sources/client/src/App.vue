<template>
    <div class="card flex justify-content-center">
        <Button label="Check" icon="pi pi-check" @click="click" />
        <Button label="Create" @click="create" />
        <h2>{{ result }}</h2>
        <div v-if="createdProject">
            <h3>{{ createdProject.id }}</h3>
            <h3>{{ createdProject.name }}</h3>
            <h3>{{ createdProject.createdAt }}</h3>
        </div>
    </div>
</template>

<script setup lang="ts">
import Button from "primevue/button";
import * as commands from "./bindings";
import { ref } from "vue";

const result = ref<string>();
const createdProject = ref<commands.Project>();

async function click(): Promise<void> {
    result.value = await commands.greet({ name: "Keck" });
}

async function create(): Promise<void> {
    createdProject.value = await commands.createProject({ name: "keckw" });
}
</script>
