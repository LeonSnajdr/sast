<template>
    <VSystemBar style="z-index: 10000" data-tauri-drag-region window>
        <BaseBtnIcon @click="$router.back()" icon="mdi-arrow-left" size="x-small" />
        <BaseBtnIcon @click="$router.forward()" icon="mdi-arrow-right" size="x-small" />
        <VSpacer />
        <BaseBtnIcon @click="appWindow.minimize()" icon="mdi-minus" size="x-small" />
        <BaseBtnIcon
            @click="appWindow.toggleMaximize()"
            :icon="isMaximized ? 'mdi-checkbox-multiple-blank-outline' : 'mdi-checkbox-blank-outline'"
            size="x-small"
        />
        <VHover v-slot="{ isHovering, props }">
            <BaseBtnIcon
                @click="appWindow.close()"
                v-bind="props"
                :color="isHovering ? 'error' : ''"
                :variant="isHovering ? 'flat' : 'text'"
                icon="mdi-close"
                size="x-small"
            />
        </VHover>
    </VSystemBar>
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

const isMaximized = ref(false);

let unListenResizedEvent: UnlistenFn;

onBeforeMount(async () => {
    unListenResizedEvent = await appWindow.onResized(async () => {
        isMaximized.value = await appWindow.isMaximized();
    });
});

onBeforeUnmount(() => {
    unListenResizedEvent();
});
</script>
