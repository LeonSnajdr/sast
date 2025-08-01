<template>
    <VNavigationDrawer v-model="isDrawerOpen" location="right" width="450" disableResizeWatcher>
        <VList class="h-100 d-flex flex-column">
            <VListItem height="48">
                <slot name="title" />
                <template #append>
                    <VBtn @click="isDrawerOpen = false" class="mr-2" prependIcon="mdi-close">{{ $t("action.close") }}</VBtn>
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
