<template>
    <div v-show="false" id="teleports" class="position-absolute" />

    <template v-for="ptySession in ptySessions" :key="ptySession.sessionId">
        <AdaptiveTeleport :to="`#pty-session-${ptySession.sessionId}`">
            <PtySessionTerminal :sessionId="ptySession.sessionId" />
        </AdaptiveTeleport>
    </template>
</template>

<script setup lang="ts">
const ptySessionStore = usePtySessionStore();

const { ptySessions } = storeToRefs(ptySessionStore);
</script>
