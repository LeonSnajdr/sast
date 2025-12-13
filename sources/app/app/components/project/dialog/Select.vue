<template>
    <VDialog v-model="isDialogOpen" activator="parent" height="400" width="800">
        <VCard class="h-100">
            <VCardTitle>
                <VIcon color="warning" icon="mdi-folder-marker" />
                {{ $t("title.select", { type: $t("project.singular") }) }}
            </VCardTitle>
            <VCardText class="h-100 overflow-y-hidden">
                <VRowSingle>
                    <VTextField
                        v-model="query"
                        :label="$t('search.filter')"
                        :persistentPlaceholder="false"
                        appendInnerIcon="mdi-filter-outline"
                        autofocus
                        clearable
                    />
                </VRowSingle>
                <VRowSingle class="h-100 overflow-y-scroll">
                    <VList>
                        <VListItem
                            v-for="projectResult in projectResults"
                            :key="projectResult.item.id"
                            @click="isDialogOpen = false"
                            :to="getSwitchProjectLocationRef(projectResult.item).value"
                        >
                            <VListItemTitle>
                                <VIcon v-if="projectResult.item.favorite" color="warning" icon="mdi-star" size="small" />
                                {{ projectResult.item.name }}
                            </VListItemTitle>
                            <VListItemSubtitle>
                                {{ $t("date.opened", { date: useLocaleTimeAgo(projectResult.item.dateLastOpened).value }) }}
                            </VListItemSubtitle>
                            <template #append>
                                <VIcon icon="mdi-arrow-right" />
                            </template>
                        </VListItem>

                        <VListSubheader v-if="projectResults.length == 0" class="text-center">{{ $t("search.noResults") }}</VListSubheader>
                    </VList>
                </VRowSingle>
            </VCardText>
            <VCardActions>
                <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
const { getSwitchProjectLocationRef } = useProject();
const { query, projectResults } = useProjectSearch();

const isDialogOpen = ref(false);

watch(isDialogOpen, () => {
    query.value = "";
});
</script>
