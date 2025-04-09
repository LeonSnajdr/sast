<template>
    <VNavigationDrawer location="left" permanent>
        <VList class="mt-n2">
            <VListItem height="48">
                <VListItemTitle>
                    <span class="text-body-1">{{ $t("setting.title") }}</span>
                </VListItemTitle>
                <template #append>
                    <BaseBtnIcon @click="closeSetting()" icon="mdi-close" size="x-small">
                        <VTooltip activator="parent">{{ $t("action.close") }}</VTooltip>
                    </BaseBtnIcon>
                </template>
            </VListItem>
            <VDivider />
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
        },
        {
            icon: "mdi-gesture",
            name: t("setting.behavior"),
            to: { name: "index-setting-index-behavior" }
        },
        {
            icon: "mdi-information-outline",
            name: t("setting.about"),
            to: { name: "index-setting-index-about" }
        }
    ];
});

const closeSetting = () => {
    navigateTo("/");
};
</script>
