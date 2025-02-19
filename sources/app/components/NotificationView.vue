<template>
    <div class="position-absolute bottom-0 right-0 ma-4 d-flex flex-column ga-2" style="width: 500px; z-index: 9999">
        <template v-for="notification in notifications" :key="notification.id">
            <VAlert :borderColor="notification.type" border="start" class="mb-2 elevation-1" color="surface" density="compact">
                <p>{{ notification.text }}</p>
                <p v-if="notification.expanded" class="text-medium-emphasis text-caption">{{ notification.expandableText }}</p>
                <template #append>
                    <ExpandBtn v-if="notification.expandableText" v-model="notification.expanded" />
                    <IconBtn @click="notification.remove()" icon="mdi-close" />
                </template>
            </VAlert>
        </template>
    </div>
</template>

<script setup lang="ts">
const { notifications } = useNotify();
</script>
