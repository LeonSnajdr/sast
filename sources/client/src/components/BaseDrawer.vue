<template>
    <v-navigation-drawer width="200" permanent left floating>
        <template #prepend>
            <div class="d-flex">
                <slot name="headerLeft" />
                <v-spacer />
                <slot name="headerRight" />
            </div>

            <v-divider />
        </template>
        <template #default>
            <slot name="default" />
        </template>
        <template #append>
            <div class="d-flex justify-center">
                <p class="text-caption text-secondary">v {{ version }}</p>
            </div>
        </template>
    </v-navigation-drawer>
</template>

<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { computedAsync } from "@vueuse/core";

const version = computedAsync(async () => {
    return await getVersion();
}, "");
</script>
