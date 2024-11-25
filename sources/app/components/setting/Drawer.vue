<template>
    <VNavigationDrawer location="left" permanent>
        <VList class="mt-n2">
            <VListItem v-for="page in subPages" :key="page.name" :title="page.name" :to="page.to" link>
                <template #prepend>
                    <VIcon :icon="page.icon" size="small" />
                </template>
            </VListItem>
        </VList>
    </VNavigationDrawer>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

const { t } = useI18n();

const subPages = computed((): { icon: string; name: string; to: RouteLocationRaw }[] => {
    return [
        {
            icon: "mdi-auto-fix",
            name: t("setting.presentation"),
            to: { name: "index-setting-index-presentation" }
        }
    ];
});
</script>
