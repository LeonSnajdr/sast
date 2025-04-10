<template>
    <VRowSingle>
        <VCard>
            <VCardText class="d-flex">
                <VListItem href="https://github.com/LeonSnajdr" prependAvatar="assets/img/author.jpg" target="_blank">
                    <VListItemTitle>
                        {{ $t("setting.about.appInfo.author.title") }}
                    </VListItemTitle>
                    <VListItemSubtitle>Leon Snajdr</VListItemSubtitle>
                </VListItem>
                <VListItem href="https://github.com/LeonSnajdr/sast" target="_blank">
                    <VListItemTitle>
                        {{ $t("setting.about.appInfo.version.title") }}
                    </VListItemTitle>
                    <VListItemSubtitle>{{ version }} </VListItemSubtitle>
                </VListItem>
                <VListItem href="https://v2.tauri.app/" target="_blank">
                    <VListItemTitle>
                        {{ $t("setting.about.appInfo.tauriVersion.title") }}
                    </VListItemTitle>
                    <VListItemSubtitle> {{ tauriVersion }}</VListItemSubtitle>
                </VListItem>
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
import { getVersion, getTauriVersion } from "@tauri-apps/api/app";

const version = computedAsync(async () => {
    return await getVersion();
}, "-");

const tauriVersion = computedAsync(async () => {
    return await getTauriVersion();
}, "-");
</script>
