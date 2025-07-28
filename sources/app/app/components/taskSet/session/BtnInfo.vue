<template>
    <VBtn class="ml-2" size="28">
        <BaseIconIndeterminate v-if="hasRunningTaskSetSessions" :size="18" color="warning" icon="mdi-circle" indeterminate />
        <BaseIconIndeterminate v-else-if="isLastTaskSetSesionFailed" :size="18" color="error" icon="mdi-close-circle" />
        <BaseIconIndeterminate v-else :size="18" icon="mdi-checkbox-multiple-marked-circle" />
    </VBtn>
</template>

<script setup lang="ts">
const taskSetSessionStore = useTaskSetSessionStore();

const { sessions } = storeToRefs(taskSetSessionStore);

const hasRunningTaskSetSessions = computed(() => {
    return sessions.value.some((x) => x.status == "Running");
});

const isLastTaskSetSesionFailed = computed(() => {
    return lodMaxBy(sessions.value, (x) => new Date(x.dateStarted))?.status === "Failed";
});
</script>
