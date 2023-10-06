<template>
    <v-layout>
        <v-main class="h-screen text-body-2">
            <ProjectList />
            <slot></slot>
        </v-main>
    </v-layout>
</template>

<script setup lang="ts">
import ProjectList from "@/views/project/ProjectList.vue";
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
