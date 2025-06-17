<template>
    <VNavigationDrawer v-model="isDrawerOpen" location="right" width="450" disableResizeWatcher>
        <VList class="h-100 d-flex flex-column">
            <VListItem height="48">
                <slot name="title" />
                <template #append>
                    <BaseBtnIcon @click="isDrawerOpen = false" class="mr-2" icon="mdi-close">{{ $t("action.close") }}</BaseBtnIcon>
                    <slot name="actions" />
                </template>
            </VListItem>
            <VDivider />
            <VContainer class="flex-grow-1 overflow-auto">
                <slot name="content" />
            </VContainer>
        </VList>
    </VNavigationDrawer>
</template>

<script setup lang="ts">
const isDrawerOpen = defineModel<boolean>({ required: true });
</script>
