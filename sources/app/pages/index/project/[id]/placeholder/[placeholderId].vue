<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("title.edit", { type: $t("placeholder.singular") }) }}</VAppBarTitle>
        <PlaceholderActionDelete v-if="placeholder" :placeholder="placeholder" class="mr-2" />
        <PlaceholderActionSave v-if="placeholder" :disabled="!isFormValid" :placeholder="placeholder" />
    </VAppBar>
    <VContainer>
        <VCard :loading="isLoading">
            <VCardText v-if="placeholder">
                <VForm v-model="isFormValid">
                    <PlaceholderFieldName v-model="placeholder.name" />
                    <PlaceholderFieldValue v-model="placeholder.value" />
                    <PlaceholderFieldVisibility v-model="placeholder.visibility" />
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
