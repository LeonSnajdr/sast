<template>
    <VAppBar class="d-flex ga-4">
        <VAppBarTitle>{{ $t("placeholder.edit.title") }}</VAppBarTitle>
        <div class="d-flex ga-2">
            <IconBtn color="error" icon="mdi-delete" />
            <IconBtn color="success" icon="mdi-content-save" variant="flat" />
        </div>
    </VAppBar>
    <VContainer>
        {{ route.params.placeholderId }}
    </VContainer>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-placeholder-placeholderId");

const isLoading = ref(false);
const placeholder = ref<PlaceholderContract>();

onBeforeMount(() => {
    loadPlaceholder();
});

const loadPlaceholder = async () => {
    isLoading.value = true;

    const placeholderResult = await commands.getPlaceholder(route.params.placeholderId);

    isLoading.value = false;

    if (placeholderResult.status === "error") {
        return;
    }

    placeholder.value = placeholderResult.data;
};
</script>
