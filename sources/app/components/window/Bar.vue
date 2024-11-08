<template>
    <VSystemBar style="z-index: 10000" data-tauri-drag-region window>
        <IconBtn @click="$router.back()" icon="mdi-arrow-left" />
        <IconBtn @click="$router.forward()" icon="mdi-arrow-right" />
        <VSpacer />
        {{ $route.path }}
        <VSpacer />
        <IconBtn @click="appWindow.minimize()" icon="mdi-minus" />
        <IconBtn @click="appWindow.toggleMaximize()" :icon="isMaximized ? 'mdi-checkbox-multiple-blank-outline' : 'mdi-checkbox-blank-outline'" />
        <VHover v-slot="{ isHovering, props }">
            <IconBtn @click="appWindow.close()" v-bind="props" :color="isHovering ? 'error' : ''" :variant="isHovering ? 'flat' : 'text'" icon="mdi-close" />
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
