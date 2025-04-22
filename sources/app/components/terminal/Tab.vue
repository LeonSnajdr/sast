<template>
    <VBtn
        :to="{ name: 'index-project-id-terminal-terminalId', params: { id: route.params.id, terminalId: terminal.id } }"
        class="ml-2 px-2 text-body-2"
        density="comfortable"
    >
        <template #prepend>
            <div class="d-flex ga-1">
                <VBadge :color="indicator.color" location="bottom right" offsetX="-2" offsetY="-2" dot>
                    <VIcon color="info" icon="mdi-powershell" />
                    <VTooltip v-if="indicator.tooltip" :text="indicator.tooltip" activator="parent" location="left" />
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
            <TerminalActionClose :terminal />
        </template>
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    terminal: TerminalInfoContract;
}>();

const route = useRoute("index-project-id-terminal-terminalId");

const indicator = computed(() => {
    const status = props.terminal.shellStatus;

    if (typeof status === "object" && "Crashed" in status) {
        return { color: "error", tooltip: `${status.Crashed.code} ${status.Crashed.message}` };
    } else if (status == "Restarting") {
        return { color: "warning" };
    } else if (status == "Running") {
        return { color: "success" };
    }

    return { color: undefined };
});
</script>
