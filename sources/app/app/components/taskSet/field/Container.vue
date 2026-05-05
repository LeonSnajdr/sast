<template>
    <VForm ref="form" v-model="isValid">
        <VRowSingle colClass="d-flex align-center ga-2">
            <TaskSetFieldName v-model="taskSet.name" autofocus />
            <TaskSetFieldFavorite v-model="taskSet.favorite" mode="button" />
        </VRowSingle>
    </VForm>
</template>

<script setup lang="ts">
const taskSet = defineModel<TaskSetCreateContract | TaskSetContract>({ required: true });
const isValid = defineModel<boolean | null>("isValid", { required: true });
</script>
