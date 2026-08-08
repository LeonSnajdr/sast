<template>
    <VSystemBar class="d-flex" style="z-index: 10000" data-tauri-drag-region window>
        <div class="d-flex" style="min-width: 200px" data-tauri-drag-region window>
            <template v-if="currentOs !== 'macos'">
                <VIconBtn @click="$router.back()" icon="mdi-arrow-left" size="small" />
                <VIconBtn @click="$router.forward()" icon="mdi-arrow-right" size="small" />
            </template>
        </div>
        <div class="overflow-auto" data-tauri-drag-region>
            <div id="window-bar-content" />
        </div>
        <VSpacer data-tauri-drag-region />
        <div v-if="currentOs !== 'macos'" class="d-flex ga-2">
            <VIconBtn @click="appWindow.minimize()" icon="mdi-minus" size="small" />
            <VIconBtn
                @click="appWindow.toggleMaximize()"
                :icon="isMaximized ? 'mdi-checkbox-multiple-blank-outline' : 'mdi-checkbox-blank-outline'"
                size="small"
            />
            <VHover v-slot="{ isHovering, props }">
                <VIconBtn
                    @click="appWindow.close()"
                    v-bind="props"
                    :color="isHovering ? 'error' : ''"
                    :variant="isHovering ? 'flat' : 'text'"
                    icon="mdi-close"
                    size="small"
                />
            </VHover>
        </div>
    </VSystemBar>
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from "@tauri-apps/plugin-os";

const appWindow = getCurrentWindow();
const currentOs = platform();

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
