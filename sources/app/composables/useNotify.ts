const notifications = ref<{ text: string; color: string }[]>([]);

export default function useNotify() {
    const success = (text: string) => {
        notifications.value.push({
            text: text,
            color: "success"
        });
    };

    const warning = (text: string) => {
        notifications.value.push({
            text: text,
            color: "warning"
        });
    };

    const error = (text: string) => {
        notifications.value.push({
            text: text,
            color: "error"
        });
    };

    const info = (text: string) => {
        notifications.value.push({
            text: text,
            color: "info"
        });
    };

    return { notifications, success, warning, error, info };
}
