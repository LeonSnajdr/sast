<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("taskSet.plural") }}</VAppBarTitle>
        <BaseBtnIcon icon="mdi-plus" variant="tonal">
            {{ $t("action.create") }}
            <TaskSetDialogCreate />
        </BaseBtnIcon>
    </VAppBar>

    <VContainer class="h-100">
        <VCard>
            <VCardText>
                <TaskSetTable :loading="isLoading" :taskSets="taskSets" />
            </VCardText>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
const taskSetStore = useTaskSetStore();

const { isLoading, taskSets } = storeToRefs(taskSetStore);
</script>
