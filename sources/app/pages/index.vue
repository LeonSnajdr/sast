<template>
    <VContainer>
        <VCard>
            <VCardTitle>This is my card</VCardTitle>
            <VCardText> Hallo, ich bin der roli </VCardText>
            <VBtn @click="callRust">Call</VBtn>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const callRust = async () => {
    const keckw = await invoke("get_all_projects");

    console.log(keckw);
};
</script>
