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
                <template #prepend="{ isActive }">
                    <VIcon :icon="isActive ? page.iconActive : page.icon" size="small" />
                </template>
            </VListItem>
        </VList>
    </VNavigationDrawer>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

const { t } = useI18n();

const subPages = computed((): { icon: string; iconActive: string; name: string; to: RouteLocationRaw }[] => {
    return [
        {
            icon: "mdi-auto-fix",
            iconActive: "mdi-auto-fix",
            name: t("setting.presentation"),
            to: { name: "index-setting-index-presentation" }
        },
        {
            icon: "mdi-gesture",
            iconActive: "mdi-gesture",
            name: t("setting.behavior"),
            to: { name: "index-setting-index-behavior" }
        },
        {
            icon: "mdi-information-outline",
            iconActive: "mdi-information",
            name: t("setting.about"),
            to: { name: "index-setting-index-about" }
        }
    ];
});

const closeSetting = () => {
    navigateTo("/");
};
</script>
