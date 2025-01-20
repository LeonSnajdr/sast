<template>
    <div class="fill-height d-flex flex-column">
        <VAppBar>
            <VAppBarTitle>{{ t("placeholder.title") }}</VAppBarTitle>
            <IconBtn color="secondary" icon="mdi-plus" variant="flat">
                <PlaceholderCreateDialog @created="loadPlaceholders()" />
            </IconBtn>
        </VAppBar>
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
const globalPlaceholders = ref<PlaceholderContract[]>([]);
const projectPlaceholders = ref<PlaceholderContract[]>([]);

onBeforeMount(() => {
    loadPlaceholders();
});

const loadPlaceholders = async () => {
    isLoading.value = true;

    const globalPlaceholdersResult = await commands.getGlobalPlaceholders();
    const projectPlaceholdersResult = await commands.getProjectPlaceholders(selectedProject.value.id);

    isLoading.value = false;

    if (globalPlaceholdersResult.status === "error" || projectPlaceholdersResult.status === "error") {
        notify.error(t("placeholder.load.error"));
        return;
    }

    globalPlaceholders.value = globalPlaceholdersResult.data;
    projectPlaceholders.value = projectPlaceholdersResult.data;
};
</script>
