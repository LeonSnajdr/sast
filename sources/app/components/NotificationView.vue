<template>
    <template v-for="(notification, index) in activeNotifications" :key="notification.id">
        <VSnackbar
            v-model="notification.active"
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
                <IconBtn @click="notification.active = false" icon="mdi-close" />
            </template>
        </VSnackbar>
    </template>
</template>

<script setup lang="ts">
const { notifications } = useNotify();

const activeNotifications = computed(() => {
    return notifications.value.filter((x) => x.active);
});
</script>

<style lang="scss">
.v-snackbar__wrapper {
    &.success {
        border-left: 3px solid rgb(var(--v-theme-success));
    }

    &.info {
        border-left: 3px solid rgb(var(--v-theme-info));
    }

    &.warning {
        border-left: 3px solid rgb(var(--v-theme-warning));
    }

    &.error {
        border-left: 3px solid rgb(var(--v-theme-error));
    }
}
</style>
