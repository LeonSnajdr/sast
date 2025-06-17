<template>
    <VBtn
        :to="{ name: 'index-project-id-terminal-terminalId', params: { id: route.params.id, terminalId: terminal.id } }"
        class="ml-2 px-2 text-body-2"
        density="comfortable"
    >
        <template #prepend>
            <div class="d-flex ga-1">
                <TerminalShellStatusBadge :shellStatus="terminal.shellStatus" offsetX="-2" offsetY="-2">
                    <VIcon color="info" icon="mdi-powershell" />
                </TerminalShellStatusBadge>

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
defineProps<{
    terminal: TerminalInfoContract;
}>();

const route = useRoute("index-project-id-terminal-terminalId");
</script>
