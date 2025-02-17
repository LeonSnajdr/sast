<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("placeholder.plural") }}</VAppBarTitle>
        <PlaceholderActionCreate />
    </VAppBar>

    <div class="fill-height d-flex flex-column">
        <VContainer class="flex-grow-1 d-flex flex-column ga-4">
            <VCard>
                <VCardTitle>
                    <PlaceholderIcon visibility="Global" />
                    {{ $t("placeholder.card.global.title") }}
                </VCardTitle>
                <VCardText>
                    <PlaceholderTable :loading="isLoading" :placeholders="globalPlaceholders" />
                </VCardText>
            </VCard>
            <VCard>
                <VCardTitle>
                    <PlaceholderIcon visibility="Project" />
                    {{ $t("placeholder.card.global.project") }}
                </VCardTitle>
                <VCardText>
                    <PlaceholderTable :loading="isLoading" :placeholders="projectPlaceholders" />
                </VCardText>
            </VCard>
        </VContainer>
    </div>
</template>

<script setup lang="ts">
const projectStore = usePlaceholderStore();

const { isLoading, placeholders } = storeToRefs(projectStore);

const globalPlaceholders = computed(() => {
    return placeholders.value.filter((x) => x.visibility === "Global");
});

const projectPlaceholders = computed(() => {
    return placeholders.value.filter((x) => x.visibility === "Project");
});
</script>
