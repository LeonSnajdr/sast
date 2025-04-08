<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("task.plural") }}</VAppBarTitle>
        <BaseBtnIcon color="secondary" icon="mdi-plus" variant="flat">
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
