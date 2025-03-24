<template>
    <VBtn
        :to="{ name: 'index-project-id-pty-sessionId', params: { id: route.params.id, sessionId: terminal.id } }"
        class="ml-2 px-2 text-body-2"
        density="comfortable"
    >
        <template #prepend>
            <VIcon color="info" icon="mdi-powershell" />
            <VBtn v-if="terminal.task" @click.prevent.stop color="success" size="20" variant="plain">
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
