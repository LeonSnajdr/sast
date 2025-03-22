<template>
    <VSlideGroup showArrows>
        <VSlideGroupItem v-for="terminal in terminals" :key="terminal.id">
            <VBtn
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
        </VSlideGroupItem>
    </VSlideGroup>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-pty-sessionId");
const terminalStore = useTerminalStore();

const { terminals } = storeToRefs(terminalStore);
</script>
