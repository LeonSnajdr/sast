const notifications = ref<{ id: string; active: boolean; expanded: boolean; type: string; text: string; remove: () => void; expandableText?: string }[]>([]);

export default function useNotify() {
    const success = (text: string) => {
        addNotification("success", text, 3000);
    };

    const info = (text: string) => {
        addNotification("info", text, 5000);
    };

    const warning = (text: string) => {
        addNotification("warning", text, 5000);
    };

    const error = (text: string, error?: unknown) => {
        addNotification("error", text, -1, getCommandError(error));
    };

    const getCommandError = (error?: unknown) => {
        if (error && typeof error === "object") {
            if ("Db" in error) {
                const dbError = error as { Db: string };
                return dbError.Db;
            }
        }
    };

    const addNotification = (type: string, text: string, timeout: number, expandableText?: string) => {
        const id = crypto.randomUUID();

        const remove = () => {
            lodRemove(notifications.value, (x) => x.id === id);
        };

        notifications.value.push({
            id,
            active: true,
            expanded: false,
            type,
            text,
            remove,
            expandableText
        });

        if (timeout > 0) {
            setTimeout(() => {
                remove();
            }, timeout);
        }
    };

    return { notifications, success, warning, error, info };
}
