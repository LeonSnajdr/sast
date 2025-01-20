<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("placeholder.edit.title") }}</VAppBarTitle>
        <PlaceholderActionDelete v-if="placeholder" :placeholder="placeholder" />
        <VBtn :disabled="!isFormValid" color="success" density="comfortable" variant="flat">{{ $t("action.save") }}</VBtn>
    </VAppBar>
    <VContainer>
        <VCard :loading="isLoading">
            <VCardText v-if="placeholder">
                <VForm ref="form" v-model="isFormValid">
                    <PlaceholderFieldName v-model="placeholder.name" />
                    <PlaceholderFieldValue v-model="placeholder.value" />
                    <PlaceholderFieldProjectId v-model="placeholder.projectId" />
                </VForm>
            </VCardText>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-placeholder-placeholderId");

const isLoading = ref(false);
const isFormValid = ref(false);
const placeholder = ref<PlaceholderContract>();

onBeforeMount(() => {
    loadPlaceholder();
});

const loadPlaceholder = async () => {
    isLoading.value = true;

    const placeholderResult = await commands.placeholderGetOne(route.params.placeholderId);

    isLoading.value = false;

    if (placeholderResult.status === "error") {
        return;
    }

    placeholder.value = placeholderResult.data;
};
</script>
