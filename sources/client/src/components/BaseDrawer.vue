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
    console.log("get verison");

    return await getVersion();
}, "");
</script>

<style lang="scss" scoped>
.v-list-item {
    :deep(.v-list-item__overlay) {
        opacity: 1;
        background-color: transparent;
    }

    &:hover {
        :deep(.v-list-item__overlay) {
            background-color: rgba(var(--v-theme-secondary), 0.2);
        }
    }

    &--active {
        :deep(.v-list-item__overlay) {
            background-color: rgba(var(--v-theme-secondary), 0.2);
            border-left: 3px solid rgb(var(--v-theme-primary));
        }
    }
}
</style>
