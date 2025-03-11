<template>
    <VSlideGroup showArrows>
        <VSlideGroupItem v-for="ptySession in ptySessions" :key="ptySession.id">
            <VBtn
                :to="{ name: 'index-project-id-pty-sessionId', params: { id: route.params.id, sessionId: ptySession.id } }"
                class="ml-2 px-2 text-body-2"
                density="comfortable"
            >
                <template #prepend>
                    <VIcon color="info" icon="mdi-powershell" />
                </template>
                <span class="text-truncate" style="max-width: 150px">{{ ptySession.name }}</span>
                <template #append>
                    <PtySessionActionKill :id="ptySession.id" />
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
