<template>
    <VBtn
        :to="{ name: 'index-project-id-pty-sessionId', params: { id: route.params.id, sessionId: terminal.id } }"
        class="ml-2 px-2 text-body-2"
        density="comfortable"
    >
        <template #prepend>
            <div class="d-flex ga-1">
                <VBadge :color="indicatorColor" location="bottom right" offsetX="-2" offsetY="-2" dot>
                    <VIcon color="info" icon="mdi-powershell" />
                </VBadge>
                <VBtn v-if="terminal.task" @click.prevent.stop size="20" variant="plain">
                    <VIcon icon="mdi-checkbox-marked-circle-outline" size="small" />
                    <VMenu activator="parent" openOnHover>
                        <VList>
                            <VListItem :subtitle="$t('action.restart.description', { name: terminal.task.name })" :title="$t('action.restart')">
                                <template #prepend>
                                    <TaskActionRestart :task="terminal.task" />
                                </template>
                            </VListItem>
                        </VList>
                    </VMenu>
                </VBtn>
            </div>
        </template>
        <span class="text-truncate" style="max-width: 150px">{{ terminal.name }}</span>
        <template #append>
            <TerminalActionKill v-if="isKillable" :id="terminal.id" />
            <TerminalActionDelete v-if="isClosable" :id="terminal.id" />
        </template>
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    terminal: TerminalInfoContract;
}>();

const route = useRoute("index-project-id-pty-sessionId");

const isKillable = computed(() => {
    return props.terminal.shellStatus === "Running";
});

const isClosable = computed(() => {
    return props.terminal.shellStatus === "Killed" || props.terminal.shellStatus === "Failed";
});

const indicatorColor = computed(() => {
    switch (props.terminal.shellStatus) {
        case "Killed":
            return undefined;
        case "Failed":
            return "error";
        default:
            return "success";
    }
});
</script>
