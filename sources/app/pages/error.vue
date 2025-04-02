<template>
    <VEmptyState :headline="$t('error.headline')" :text="$t('error.message')" color="error" icon="mdi-alert-circle">
        <VBtn @click="reloadNuxtApp({ path: '/' })" color="primary">{{ $t("error.button") }}</VBtn>
    </VEmptyState>
</template>
