<template>
    <VBtn
        :to="{ name: 'index-project-id-pty-sessionId', params: { id: route.params.id, sessionId: terminal.id } }"
        class="ml-2 px-2 text-body-2"
        density="comfortable"
    >
        <template #prepend>
            <div class="d-flex ga-1">
                <VBadge :color="terminal.shellStatus === 'Running' ? 'success' : 'error'" location="bottom right" offsetX="-2" offsetY="-2" dot>
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
            <TerminalActionKill v-if="terminal.shellStatus === 'Running'" :id="terminal.id" />
            <TerminalActionDelete v-if="terminal.shellStatus === 'Killed'" :id="terminal.id" />
        </template>
    </VBtn>
</template>

<script setup lang="ts">
defineProps<{
    terminal: TerminalInfoContract;
}>();

const route = useRoute("index-project-id-pty-sessionId");
</script>
