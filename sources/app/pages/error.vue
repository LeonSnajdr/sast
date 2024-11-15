<template>
    <VEmptyState :headline="$t('error.headline')" :text="$t('error.message')" color="error" icon="mdi-alert-circle">
        <VBtn @click="navigateTo({ name: 'index' })" color="primary">{{ $t("error.button") }}</VBtn>
    </VEmptyState>
</template>
