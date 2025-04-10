<template>
    <VRowSingle>
        <VCard>
            <VCardTitle>
                <VIcon color="info" icon="mdi-update" />
                {{ $t("setting.about.update.title") }}
                <VSpacer />
                <BaseBtnIcon @click="updateStore.loadUpdate()" icon="mdi-refresh" />
            </VCardTitle>
            <VCardText>
                <BaseEmptyStateLoader v-if="status === UpdateStatus.Loading" />
                <template v-else>
                    <div v-if="updateInfo">
                        <VRowSingle>
                            {{ $t("setting.about.update.some") }}
                            <VChip color="success" density="comfortable" variant="tonal">{{ updateInfo.version }}</VChip>
                        </VRowSingle>
                        <VRowSingle>
                            <VProgressLinear v-model="downloaded" :max="contentLength" color="primary" />
                        </VRowSingle>
                        <VRowSingle>
                            <BaseBtnIcon @click="updateStore.download()" v-bind="downloadButtonProps">{{ $t("action.download") }}</BaseBtnIcon>
                            <BaseBtnIcon @click="updateStore.install()" v-bind="installButtonProps" class="ml-2">{{ $t("action.install") }}</BaseBtnIcon>
                        </VRowSingle>
                    </div>
                    <div v-else class="text-center">
                        <VIcon color="success" icon="mdi-check" />
                        {{ $t("setting.about.update.none") }}
                    </div>
                </template>
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
const updateStore = useUpdateStore();

const { updateInfo, status, downloaded, contentLength } = storeToRefs(updateStore);

const downloadButtonProps = computed(() => ({
    color: status.value === UpdateStatus.UpdateAvailable ? "primary" : "",
    variant: status.value === UpdateStatus.UpdateAvailable ? "flat" : "text",
    loading: status.value === UpdateStatus.Downloading,
    disabled: status.value !== UpdateStatus.UpdateAvailable
}));

const installButtonProps = computed(() => ({
    color: status.value === UpdateStatus.Downloaded ? "primary" : "",
    variant: status.value === UpdateStatus.Downloaded ? "flat" : "text",
    loading: status.value === UpdateStatus.Installing,
    disabled: status.value !== UpdateStatus.Downloaded
}));
</script>
