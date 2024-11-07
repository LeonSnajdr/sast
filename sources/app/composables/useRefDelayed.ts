// returns new ref for value, that has some delay before value is changed to false
// can be used to ensure progress indicator is shown for some minimum time
export default function useRefDelayed(value: Ref<boolean>, delay: number, delayValue = false) {
    if (delay <= 0) return value;

    const delayed: Ref<boolean> = ref(value.value as boolean) as Ref<boolean>;
    const timer = ref<number>();

    const updater = (newValue: boolean) => {
        if (newValue !== delayValue) {
            if (timer.value) window.clearTimeout(timer.value);
            delayed.value = newValue;
        } else {
            timer.value = window.setTimeout(() => (delayed.value = newValue), delay);
        }
    };

    watch(value, (newValue: boolean) => updater(newValue));

    return delayed;
}
