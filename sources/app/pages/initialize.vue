<template>
    <VContainer class="d-flex flex-column align-content-center" maxWidth="800">
        <SettingPresentationLanguage />
        <SettingPresentationTheme />
        <VRowSingle colClass="d-flex">
            <VSpacer />
            <VBtn @click="initializeSetting()" appendIcon="mdi-arrow-right" color="primary">Los gehts</VBtn>
        </VRowSingle>
    </VContainer>
</template>

<script setup lang="ts">
const initializeSetting = () => {
    navigateTo({ name: "welcome" });
};
</script>
