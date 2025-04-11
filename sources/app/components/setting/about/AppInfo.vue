<template>
    <VRowSingle>
        <VCard>
            <VCardText>
                <VListItem href="https://github.com/LeonSnajdr" target="_blank">
                    <template #prepend>
                        <VImg aspectRatio="1/1" class="mr-4" height="60" src="assets/img/logo.png" width="60" cover />
                    </template>
                    <VListItemTitle>
                        {{ $t("setting.about.appInfo.author.title") }}
                    </VListItemTitle>
                    <VListItemSubtitle>Leon Snajdr</VListItemSubtitle>
                </VListItem>
                <VRow>
                    <VCol>
                        <VList>
                            <VListItem href="https://github.com/LeonSnajdr/sast" target="_blank">
                                <VListItemTitle>
                                    {{ $t("setting.about.appInfo.version.title") }}
                                </VListItemTitle>
                                <VListItemSubtitle>
                                    {{ isDev ? $t("environment.dev") : version }}
                                </VListItemSubtitle>
                            </VListItem>
                            <VListItem href="https://v2.tauri.app/" target="_blank">
                                <VListItemTitle>
                                    {{ $t("setting.about.appInfo.tauriVersion.title") }}
                                </VListItemTitle>
                                <VListItemSubtitle>
                                    {{ tauriVersion }}
                                </VListItemSubtitle>
                            </VListItem>
                        </VList>
                    </VCol>
                    <VCol>
                        <VList>
                            <VListItem @click="revealItemInDir(logDir)">
                                <VListItemTitle>
                                    {{ $t("setting.about.appInfo.logDir.title") }}
                                </VListItemTitle>
                                <VListItemSubtitle v-tooltip="logDir">
                                    {{ logDir }}
                                </VListItemSubtitle>
                            </VListItem>
                            <VListItem @click="revealItemInDir(dataDir)">
                                <VListItemTitle>
                                    {{ $t("setting.about.appInfo.dataDir.title") }}
                                </VListItemTitle>
                                <VListItemSubtitle v-tooltip="dataDir">
                                    {{ dataDir }}
                                </VListItemSubtitle>
                            </VListItem>
                        </VList>
                    </VCol>
                </VRow>
            </VCardText>
        </VCard>
    </VRowSingle>
</template>

<script setup lang="ts">
import { getVersion, getTauriVersion } from "@tauri-apps/api/app";
import { appDataDir, appLogDir } from "@tauri-apps/api/path";
import { revealItemInDir } from "@tauri-apps/plugin-opener";

const isDev = import.meta.dev;

const version = computedAsync(async () => {
    return await getVersion();
}, "-");

const tauriVersion = computedAsync(async () => {
    return await getTauriVersion();
}, "-");

const logDir = computedAsync(async () => {
    return (await appLogDir()) + "\\sast.log";
}, "-");

const dataDir = computedAsync(async () => {
    return (await appDataDir()) + "\\sast.db";
}, "-");
</script>
