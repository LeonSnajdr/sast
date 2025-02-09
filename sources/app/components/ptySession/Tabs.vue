<template>
    <VSlideGroup showArrows>
        <VSlideGroupItem v-for="ptySession in ptySessions" :key="ptySession.sessionId">
            <VBtn
                :to="{ name: 'index-project-id-pty-sessionId', params: { id: route.params.id, sessionId: ptySession.sessionId } }"
                class="ml-2 px-2 text-body-2"
                density="comfortable"
            >
                <template #prepend>
                    <VIcon color="info" icon="mdi-powershell" />
                </template>
                <span class="text-truncate" style="max-width: 150px">{{ ptySession.name }}</span>
                <template #append>
                    <PtySessionActionKill :sessionId="ptySession.sessionId" />
                </template>
            </VBtn>
        </VSlideGroupItem>
    </VSlideGroup>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-pty-sessionId");
const ptySessionStore = usePtySessionStore();

const { ptySessions } = storeToRefs(ptySessionStore);
</script>
