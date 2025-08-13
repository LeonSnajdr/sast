<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("taskSet.plural") }}</VAppBarTitle>
        <VBtn prependIcon="mdi-plus" variant="tonal">
            {{ $t("action.create") }}
            <TaskSetDialogCreate />
        </VBtn>
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
