<template>
    <VAppBar>
        <VAppBarTitle>{{ $t("title.edit", { type: $t("placeholder.singular") }) }}</VAppBarTitle>
        <BaseActionBack />
        <PlaceholderActionDelete v-if="placeholder" :placeholder="placeholder" class="mr-2" />
        <PlaceholderActionSave v-if="placeholder" :disabled="!isPlaceholderValid" :placeholder="placeholder" />
    </VAppBar>
    <VContainer>
        <VCard :loading="isLoading" class="mb-4">
            <VCardText v-if="placeholder">
                <VListItem>
                    <VListItemTitle>{{ placeholder.name }}</VListItemTitle>
                    <VListItemSubtitle>{{ $t("date.created", { date: useLocaleTimeAgo(placeholder.dateCreated).value }) }}</VListItemSubtitle>

                    <template #append>
                        <PlaceholderFieldFavorite v-model="placeholder.favorite" mode="button" />
                    </template>
                </VListItem>
            </VCardText>
        </VCard>
        <VCard :loading="isLoading">
            <VCardText v-if="placeholder">
                <PlaceholderFieldContainer v-model="placeholder" v-model:isValid="isPlaceholderValid" />
            </VCardText>
        </VCard>
    </VContainer>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-placeholder-placeholderId");

const isLoading = ref(false);
const isPlaceholderValid = ref<boolean | null>(false);
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
