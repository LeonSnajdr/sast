<template>
    <div class="fill-height d-flex flex-column">
        <VAppBar>
            <VSlideGroup showArrows>
                <VSlideGroupItem v-for="tab in tabs" :key="tab">
                    <VBtn :to="{ name: 'index-project-id-tabs-tabId', params: { id: route.params.id, tabId: tab } }" class="ml-2 px-2" density="comfortable">
                        <template #prepend>
                            <VIcon color="info" icon="mdi-powershell" />
                        </template>
                        <span class="text-truncate" style="max-width: 150px">{{ tab }}</span>
                        <template #append>
                            <VDefaultsProvider :defaults="{ VIcon: { size: '16' } }">
                                <VBtn @click.prevent.stop="closeTab(tab)" icon="mdi-close" size="20" variant="plain" />
                            </VDefaultsProvider>
                        </template>
                    </VBtn>
                </VSlideGroupItem>
            </VSlideGroup>
            <VSpacer />
            <IconBtn color="secondary" icon="mdi-plus" variant="flat" />
        </VAppBar>

        <NuxtPage keepalive />
    </div>
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-tabs");

const tabs = ref<string[]>(["Admin", "Bot", "Dispatcher", "super", "marisooooooooooooooooooooooooooooooooo"]);

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
