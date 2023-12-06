<template>
    <v-app v-if="appInitialized">
        <v-main class="h-screen text-body-2">
            <component :is="route.meta.layout || 'div'">
                <RouterView />
            </component>
        </v-main>
    </v-app>

    <Notification />
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import Notification from "@/components/Notification.vue";

const route = useRoute();
const notify = useNotificationStore();
const settingsStore = useSettingsStore();

const appInitialized = ref(false);

onBeforeMount(() => {
    initializeApp();
});

const initializeApp = async () => {
    try {
        await settingsStore.initializeAppWithSettings();

        appInitialized.value = true;
    } catch (error) {
        console.log("Error initializing app");
        notify.error("app.initialize.failed");
    }
};
</script>

<style lang="scss">
@use "@/styles/settings";
@import "@/styles/global";

$container-padding: settings.$container-padding-x;

.v-main {
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
</style>
