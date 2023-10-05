<template>
    <VApp>
        <VMain class="h-screen text-body-2" scrollable>
            <RouterView />
        </VMain>
    </VApp>
</template>

<script setup lang="ts">
import { VMain } from "vuetify/components";
</script>
