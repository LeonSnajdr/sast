import type { Update } from "@tauri-apps/plugin-updater";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

export enum UpdateStatus {
    NotLoaded = "NotLoaded",
    Loading = "Loading",
    UpdateAvailable = "UpdateAvailable",
    NoUpdateAvailable = "NoUpdateAvailable",
    Downloading = "Downloading",
    Downloaded = "Downloaded",
    Installing = "Installing"
}

export interface UpdateInfo {
    version: string;
}

export const useUpdateStore = defineStore("update", () => {
    const notify = useNotify();
    const { t } = useI18n();

    const status = ref(UpdateStatus.NotLoaded);
    const updateInfo = ref<UpdateInfo>();

    const downloaded = ref<number>();
    const contentLength = ref<number>();

    let update: Update | null = null;

    const loadUpdate = async () => {
        try {
            status.value = UpdateStatus.Loading;
            downloaded.value = 0;
            contentLength.value = 0;

            update = await check();

            if (!update) {
                status.value = UpdateStatus.NoUpdateAvailable;
                return;
            }

            updateInfo.value = {
                version: update.version
            };

            status.value = UpdateStatus.UpdateAvailable;
        } catch (error) {
            notify.error(t("action.load.error", { type: t("update.singular") }), { error });
            status.value = UpdateStatus.NotLoaded;
        }
    };

    const notifyIfUpdateIsAvailable = async () => {
        if (!updateInfo.value) return;

        const notifyOptions: NotificaionOptionsModel = {
            timeout: -1,
            actions: [
                {
                    text: t("update.available.link"),
                    closeOnClick: true,
                    action: async () => {
                        await navigateTo({ name: "index-setting-index-about" });
                    }
                }
            ]
        };

        notify.success(t("update.available", { version: updateInfo.value.version }), notifyOptions);
    };

    const download = async () => {
        if (!update || status.value != UpdateStatus.UpdateAvailable) return;

        status.value = UpdateStatus.Downloading;

        await update.download((event) => {
            switch (event.event) {
                case "Started":
                    contentLength.value = event.data.contentLength;
                    downloaded.value = 0;
                    console.log(`started downloading ${event.data.contentLength} bytes`);
                    break;
                case "Progress":
                    downloaded.value! += event.data.chunkLength;
                    console.log(`downloaded ${downloaded.value} from ${contentLength.value}`);
                    break;
                case "Finished":
                    console.log("download finished");
                    break;
            }
        });

        status.value = UpdateStatus.Downloaded;
    };

    const install = async () => {
        if (!update || status.value != UpdateStatus.Downloaded) return;

        status.value = UpdateStatus.Installing;

        update.install();

        await relaunch();
    };

    return { updateInfo, status, downloaded, contentLength, loadUpdate, notifyIfUpdateIsAvailable, download, install };
});
