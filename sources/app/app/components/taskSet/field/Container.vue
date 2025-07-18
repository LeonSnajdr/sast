<template>
    <VForm ref="form" v-model="isValid">
        <TaskSetFieldName v-model="taskSet.name" autofocus />
    </VForm>
</template>

<script setup lang="ts">
const taskSet = defineModel<TaskSetCreateContract | TaskSetContract>({ required: true });
const isValid = defineModel<boolean | null>("isValid", { required: true });
</script>
