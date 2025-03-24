<template>
    <VBtn
        :Id="'terminal-tab-' + terminal.id"
        :to="{ name: 'index-project-id-pty-sessionId', params: { id: route.params.id, sessionId: terminal.id } }"
        class="ml-2 px-2 text-body-2"
        density="comfortable"
    >
        <template #prepend>
            <VIcon color="info" icon="mdi-powershell" />
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
