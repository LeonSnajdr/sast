<template>
    <VChip v-if="count" class="px-1" color="primary" density="comfortable" size="x-small" style="padding-bottom: 1px" label>
        {{ count }}
    </VChip>
</template>

<script setup lang="ts">
const props = defineProps<{
    project: ProjectContract;
}>();

const terminalStore = useTerminalStore();

const { allTerminals } = storeToRefs(terminalStore);

const count = computed(() => {
    const projectTerminals = allTerminals.value.filter((x) => x.projectId === props.project.id);

    if (!projectTerminals) return undefined;

    return projectTerminals.length;
});
</script>
