<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="auto">
        <VCard>
            <VCardTitle>
                <VIcon v-if="icon" :color="iconColor" :icon="icon" />
                {{ title }}
            </VCardTitle>
            <VCardText>
                {{ message }}
            </VCardText>
            <VCardActions>
                <BaseBtnIcon @click="isDialogOpen = false">{{ $t("action.cancel") }}</BaseBtnIcon>
                <BaseBtnIcon @click="emit('confirm')" color="error" variant="flat">{{ $t("action.confirm") }}</BaseBtnIcon>
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
const emit = defineEmits(["confirm"]);

defineProps<{
    title: string;
    message: string;
    icon?: string;
    iconColor?: string;
}>();

const isDialogOpen = ref(false);
</script>
