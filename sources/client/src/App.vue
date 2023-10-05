<template>
    <v-app>
        <v-main class="h-screen text-body-2" :class="route.meta.layout">
            <RouterView />
        </v-main>
    </v-app>

    <Notification />
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import { VMain } from "vuetify/components";
import Notification from "@/components/Notification.vue";

const route = useRoute();
</script>

<style lang="scss">
@use "@/styles/settings";
@import "@/styles/global";

$container-padding: settings.$container-padding-x;

.v-main {
    &.nav-left {
        margin-left: $container-padding;
        width: calc(100% - $container-padding) !important;

        .v-navigation-drawer {
            &--left {
                margin-left: $container-padding;
                margin-top: $container-padding;
                height: calc(100% - $container-padding * 2) !important;
            }
        }
    }

    &.nav-right {
        margin-right: $container-padding;
        width: calc(100% - $container-padding) !important;

        .v-navigation-drawer {
            &--right {
                margin-right: $container-padding;
                margin-top: $container-padding;
                height: calc(100% - $container-padding * 2) !important;
            }
        }
    }

    &.nav-left.nav-right {
        width: calc(100% - $container-padding * 2) !important;
    }
}
</style>
