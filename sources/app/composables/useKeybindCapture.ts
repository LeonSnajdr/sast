export default function useKeybindCapture(keybind: Ref<string>) {
    enum CaptureSate {
        None,
        Inited,
        Capturing
    }

    const captureState = ref<CaptureSate>(CaptureSate.None);
    let capturedKeys: string[] = [];

    const capture = () => {
        captureState.value = CaptureSate.Inited;
    };

    const cancel = () => {
        captureState.value = CaptureSate.None;
    };

    onKeyStroke(true, (e) => {
        if (!isCapturing.value) return;

        e.preventDefault();

        if (captureState.value === CaptureSate.Inited) {
            capturedKeys = [];
            captureState.value = CaptureSate.Capturing;

            setTimeout(() => {
                captureState.value = CaptureSate.None;
                keybind.value = capturedKeys.join("+");
            }, 100);
        }

        capturedKeys.push(e.key.toLowerCase());
    });

    const isCapturing = computed(() => {
        return [CaptureSate.Inited, CaptureSate.Capturing].includes(captureState.value);
    });

    return { isCapturing, capture, cancel };
}
