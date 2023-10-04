<template>
    <div class="font-nunito text-primary-lighten-1 box-border h-screen bg-background">
        <Toast></Toast>
        <router-view class="h-full p-4"></router-view>
    </div>
</template>

<script setup lang="ts">
import Toast from "primevue/toast";
</script>
