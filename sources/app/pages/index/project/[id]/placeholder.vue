<template>
    <div class="fill-height d-flex flex-column">
        <VToolbar>
            <VToolbarTitle>
                <p class="text-caption">Platzhalter</p>
            </VToolbarTitle>
        </VToolbar>
        <VContainer class="flex-grow-1 d-flex flex-column ga-4">
            <VCard title="Global">
                <template #prepend>
                    <VIcon color="info" icon="mdi-web" />
                </template>
                <template #append>
                    <IconBtn color="secondary" icon="mdi-plus" variant="flat">
                        <PlaceholderCreateDialog />
                    </IconBtn>
                </template>
                <VCardText>
                    <VDataTable :items="items" hideDefaultFooter />
                </VCardText>
            </VCard>
            <VCard title="Project">
                <template #prepend>
                    <VIcon color="success" icon="mdi-map-marker-radius" />
                </template>
                <template #append>
                    <IconBtn color="secondary" icon="mdi-plus" variant="flat">
                        <PlaceholderCreateDialog />
                    </IconBtn>
                </template>
                <VCardText>
                    <VDataTable :items="[]" hideDefaultFooter />
                </VCardText>
            </VCard>
        </VContainer>
    </div>
</template>

<script setup lang="ts">
const items = computed(() => {
    return [];
});
</script>
