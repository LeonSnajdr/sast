<template>
    <template v-for="(notification, index) in notifications" :key="notification.id">
        <VSnackbar
            v-model="notification.active"
            @update:modelValue="removeNotification(notification.id)"
            :contentProps="{ class: notification.type }"
            :style="{ 'padding-bottom': index * 60 + 'px' }"
            :timeout="notification.timeout"
            color="background"
            minWidth="500"
        >
            <template #default>
                <span>{{ notification.text }}</span>
            </template>
            <template #actions>
                <IconBtn @click="removeNotification(notification.id)" icon="mdi-close" />
            </template>
        </VSnackbar>
    </template>
</template>

<script setup lang="ts">
const { notifications } = useNotify();

const removeNotification = (id: string) => {
    lodRemove(notifications.value, (x) => x.id === id);
};
</script>

<style lang="scss">
.v-snackbar__wrapper {
    @each $type in success, info, warning, error {
        &.#{$type} {
            border-left: 3px solid rgb(var(--v-theme-#{$type}));
        }
    }
}
</style>
