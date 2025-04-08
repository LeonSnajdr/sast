<template>
    <VForm ref="form" v-model="isValid">
        <ProjectFieldName v-model="project.name" autofocus />
    </VForm>
</template>

<script setup lang="ts">
const project = defineModel<ProjectCreateContract | ProjectContract>({ required: true });
const isValid = defineModel<boolean | null>("isValid", { required: true });
</script>
