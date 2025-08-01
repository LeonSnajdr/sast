<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("task.plural") }}</VAppBarTitle>
        <VBtn prependIcon="mdi-plus" variant="tonal">
            {{ $t("action.create") }}
            <TaskDialogCreate />
        </VBtn>
    </VAppBar>

    <VContainer class="h-100">
        <VCard>
            <VCardText>
                <TaskTable :loading="isLoading" :tasks="tasks" />
            </VCardText>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
const taskStore = useTaskStore();

const { isLoading, tasks } = storeToRefs(taskStore);
</script>
