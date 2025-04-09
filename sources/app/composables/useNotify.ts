const notifications = ref<NotificationModel[]>([]);

export interface NotificationModel {
    id: string;
    active: boolean;
    expanded: boolean;
    type: string;
    text: string;
    remove: () => void;
    expandableText?: string;
    actions?: NotificationActionModel[];
}

export interface NotificaionOptionsModel {
    actions?: NotificationActionModel[];
    timeout?: number;
    error?: unknown;
}

export interface NotificationActionModel {
    text: string;
    closeOnClick?: boolean;
    action: () => void;
}

export default function useNotify() {
    const success = (text: string, options?: NotificaionOptionsModel) => {
        const timeout = options?.timeout ?? 3000;
        addNotification("success", text, timeout, options);
    };

    const info = (text: string, options?: NotificaionOptionsModel) => {
        const timeout = options?.timeout ?? 5000;
        addNotification("info", text, timeout, options);
    };

    const warning = (text: string, options?: NotificaionOptionsModel) => {
        const timeout = options?.timeout ?? 5000;
        addNotification("warning", text, timeout, options);
    };

    const error = (text: string, options?: NotificaionOptionsModel) => {
        const timeout = options?.timeout ?? -1;
        addNotification("error", text, timeout, options);
    };

    const addNotification = (type: string, text: string, timeout: number, options?: NotificaionOptionsModel) => {
        const id = crypto.randomUUID();

        const { error, actions } = handleOptions(options);

        const remove = () => {
            lodRemove(notifications.value, (x) => x.id === id);
        };

        notifications.value.unshift({
            id,
            active: true,
            expanded: false,
            type,
            text,
            remove,
            expandableText: error,
            actions
        });

        if (timeout > 0) {
            setTimeout(() => {
                remove();
            }, timeout);
        }
    };

    const handleOptions = (options?: NotificaionOptionsModel) => {
        if (!options) {
            return {};
        }

        const error = getCommandError(options.error);
        const actions = options.actions;

        return { error, actions };
    };
    const getCommandError = (error?: unknown) => {
        if (!error) {
            return undefined;
        }

        if (error && typeof error === "object") {
            if ("Db" in error) {
                const dbError = error as { Db: string };
                return dbError.Db;
            }
        }

        return JSON.stringify(error);
    };

    return { notifications, success, warning, error, info };
}
