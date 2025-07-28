<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("task.plural") }}</VAppBarTitle>
        <BaseBtnIcon icon="mdi-plus" variant="tonal">
            {{ $t("action.create") }}
            <TaskDialogCreate />
        </BaseBtnIcon>
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
