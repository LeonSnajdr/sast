<template>
    <VSystemBar data-tauri-drag-region window>
        <span>Sast</span>
        <VSpacer />
        <VBtn @click="appWindow.minimize()" size="x-small" variant="text">
            <VIcon icon="mdi-minus" />
        </VBtn>
        <VBtn @click="appWindow.toggleMaximize()" size="x-small" variant="text">
            <VIcon :icon="isMaximized ? 'mdi-checkbox-multiple-blank-outline' : 'mdi-checkbox-blank-outline'" size="small" />
        </VBtn>
        <VHover v-slot="{ isHovering, props }">
            <VBtn @click="appWindow.close()" v-bind="props" :color="isHovering ? 'error' : ''" :variant="isHovering ? 'flat' : 'text'" size="x-small">
                <VIcon icon="mdi-close " />
            </VBtn>
        </VHover>
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
