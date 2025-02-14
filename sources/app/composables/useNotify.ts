const notifications = ref<{ active: boolean; text: string; type: string; timeout: number; id: string }[]>([]);

export default function useNotify() {
    const success = (text: string) => {
        addNotification(text, "success", 5000);
    };

    const info = (text: string) => {
        addNotification(text, "info", 5000);
    };

    const warning = (text: string) => {
        addNotification(text, "warning", 5000);
    };

    const error = (text: string) => {
        addNotification(text, "error", 10000);
    };

    const addNotification = (text: string, type: string, timeout: number) => {
        notifications.value.push({
            active: true,
            id: crypto.randomUUID(),
            timeout: timeout,
            text: text,
            type: type
        });
    };

    return { notifications, success, warning, error, info };
}
