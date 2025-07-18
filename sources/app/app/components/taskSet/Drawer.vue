<template>
    <BaseDrawer v-model="isDrawerOpen">
        <template #title>
            {{ $t("taskSet.plural") }}
        </template>
        <template #content>
            <TaskSetTable :taskSets inline />
        </template>
    </BaseDrawer>
</template>

<script setup lang="ts">
const isDrawerOpen = defineModel<boolean>({ required: true });

const taskSetStore = useTaskSetStore();

const { taskSets } = storeToRefs(taskSetStore);
</script>
