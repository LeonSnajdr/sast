<template>
    <SettingsList />
    <v-container fluid>
        <slot></slot>
    </v-container>
</template>

<script setup lang="ts">
import SettingsList from "@/views/settings/SettingsList.vue";
</script>
