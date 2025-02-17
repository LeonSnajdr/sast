<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("placeholder.plural") }}</VAppBarTitle>
        <PlaceholderActionCreate @created="loadPlaceholders()" />
    </VAppBar>

    <div class="fill-height d-flex flex-column">
        <VContainer class="flex-grow-1 d-flex flex-column ga-4">
            <VCard>
                <VCardTitle>
                    <VIcon color="info" icon="mdi-web" />
                    {{ $t("placeholder.card.global.title") }}
                </VCardTitle>
                <VCardText>
                    <PlaceholderTable :loading="isLoading" :placeholders="globalPlaceholders" />
                </VCardText>
            </VCard>
            <VCard>
                <VCardTitle>
                    <VIcon color="success" icon="mdi-map-marker-radius" />
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
const projectStore = useProjectStore();

const notify = useNotify();
const { t } = useI18n();

const { selectedProject } = storeToRefs(projectStore);

const isLoading = ref(false);
const availablePlaceholders = ref<PlaceholderContract[]>([]);

onBeforeMount(() => {
    loadPlaceholders();
});

const loadPlaceholders = async () => {
    isLoading.value = true;

    const availablePlaceholdersResult = await commands.placeholderGetMany(selectedProject.value.id);

    isLoading.value = false;

    if (availablePlaceholdersResult.status === "error") {
        notify.error(t("placeholder.load.error"));
        return;
    }

    availablePlaceholders.value = availablePlaceholdersResult.data;
};

const globalPlaceholders = computed(() => {
    return availablePlaceholders.value.filter((x) => x.visibility === "Global");
});

const projectPlaceholders = computed(() => {
    return availablePlaceholders.value.filter((x) => x.visibility === "Project");
});
</script>
