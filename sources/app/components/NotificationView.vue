<template>
    <VSnackbarQueue v-model="notifications" location="bottom right" closable>
        <template #actions="{ props }">
            <IconBtn @click="props.onClick()" icon="mdi-close"></IconBtn>
        </template>
    </VSnackbarQueue>
</template>

<script setup lang="ts">
const { notifications } = useNotify();
</script>
