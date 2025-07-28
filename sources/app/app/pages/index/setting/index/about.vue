<template>
    <VAppBar>
        <VAppBarTitle>
            {{ $t("setting.about") }}
        </VAppBarTitle>
    </VAppBar>
    <VContainer>
        <SettingAboutAppInfo />
        <SettingAboutUpdate />
    </VContainer>
</template>
