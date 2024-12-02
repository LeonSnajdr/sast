<template>
    <div class="fill-height d-flex flex-column">
        <VToolbar border="b" height="38">
            <VSlideGroup showArrows>
                <VSlideGroupItem v-for="tab in tabs" :key="tab">
                    <VBtn :to="{ name: 'index-project-id-tabs-tabId', params: { id: route.params.id, tabId: tab } }" class="ml-2" density="compact">
                        <template #prepend>
                            <VIcon color="info" icon="mdi-powershell" />
                        </template>
                        <span>{{ tab }}</span>

                        <IconBtn @click.prevent.stop="closeTab(tab)" class="ml-2" icon="mdi-close" variant="plain" />
                    </VBtn>
                </VSlideGroupItem>
            </VSlideGroup>
            <IconBtn icon="mdi-plus" />
        </VToolbar>

        <VContainer class="flex-grow-1">
            <NuxtPage />
        </VContainer>
    </div>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-tabs");

const tabs = ref<string[]>(["Admin", "Bot", "Dispatcher", "super", "mariso"]);

const closeTab = (tab: string) => {
    const index = lodIndexOf(tabs.value, tab);

    lodRemove(tabs.value, (x) => x === tab);

    if (tabs.value.length === 0) {
        navigateTo({ name: "index-project-id-tabs", params: { id: route.params.id } });
        return;
    }

    const nextIndex = index > 0 ? index - 1 : 0;
    navigateTo({ name: "index-project-id-tabs-tabId", params: { id: route.params.id, tabId: tabs.value[nextIndex] } });
};
</script>
