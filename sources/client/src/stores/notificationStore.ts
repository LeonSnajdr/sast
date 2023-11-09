export const useNotificationStore = defineStore("notification", () => {
    const visible = ref(false);
    const message = ref("");
    const namedParams = ref<Record<string, unknown>>({});
    const notificationColor = ref("");

    const success = (text: string, named?: Record<string, unknown>) => {
        show(text, "success", named);
    };

    const warning = (text: string, named?: Record<string, unknown>) => {
        show(text, "warning", named);
    };

    const error = (text: string, named?: Record<string, unknown>) => {
        show(text, "error", named);
    };

    const info = (text: string, named?: Record<string, unknown>) => {
        show(text, "info", named);
    };

    const show = (text: string, color?: string, named?: Record<string, unknown>) => {
        notificationColor.value = color ?? "";
        namedParams.value = named ?? {};
        message.value = text;
        visible.value = true;
    };

    return { visible, message, namedParams, color: notificationColor, show, success, warning, error, info };
});
