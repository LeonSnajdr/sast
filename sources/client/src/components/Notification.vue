<template>
    <v-snackbar v-model="notify.visible" :color="notify.color">
        {{ $t(notify.message, notify.namedParams) }}
    </v-snackbar>
</template>

<script setup lang="ts">
import { useNotificationStore } from "@/stores/notificationStore";

const notify = useNotificationStore();
</script>
