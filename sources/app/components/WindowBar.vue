<template>
    <VSystemBar data-tauri-drag-region window>
        <span>Sast</span>
        <VSpacer />
        <VBtn @click="appWindow.minimize()" icon="mdi-minus" size="small" variant="text" />
        <VBtn
            @click="appWindow.toggleMaximize()"
            :icon="isMaximized ? 'mdi-checkbox-multiple-blank-outline' : 'mdi-checkbox-blank-outline'"
            size="x-small"
            variant="text"
        />
        <VBtn @click="appWindow.close()" icon="mdi-close" size="small" variant="text" />
    </VSystemBar>
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

let unListenResizedEvent: UnlistenFn;

const isMaximized = ref(false);

onBeforeMount(async () => {
    unListenResizedEvent = await appWindow.onResized(async () => {
        isMaximized.value = await appWindow.isMaximized();
    });
});

onBeforeUnmount(() => {
    unListenResizedEvent();
});
</script>
