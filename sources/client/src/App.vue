<template>
    <v-app>
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
</script>
