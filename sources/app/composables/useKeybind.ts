export default function useKeybind(andKeys: string[], action: () => void) {
    const { current: pressedKeys } = useMagicKeys();

    whenever(
        () => andKeys.every((key) => pressedKeys.has(key)),
        () => {
            action();
        }
    );
}
