<template>
    <v-app>
        <ProjectLayout v-if="route.meta.layout == 'Project'">
            <RouterView />
        </ProjectLayout>
        <DefaultLayout v-else>
            <RouterView />
        </DefaultLayout>
    </v-app>

    <Notification />
</template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import Notification from "@/components/Notification.vue";
import DefaultLayout from "@/layouts/DefaultLayout.vue";
import ProjectLayout from "@/layouts/ProjectLayout.vue";

const route = useRoute();
</script>
