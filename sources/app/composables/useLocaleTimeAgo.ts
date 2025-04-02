import type { UseTimeAgoMessages, UseTimeAgoUnitNamesDefault } from "@vueuse/core";

export function useLocaleTimeAgo(date: MaybeRefOrGetter<Date | number | string>) {
    const { t } = useI18n();

    const I18N_MESSAGES: UseTimeAgoMessages<UseTimeAgoUnitNamesDefault> = {
        justNow: t("vueUse.timeAgo.justNow"),
        past: (n) => (n.match(/\d/) ? t("vueUse.timeAgo.ago", [n]) : n),
        future: (n) => (n.match(/\d/) ? t("vueUse.timeAgo.in", [n]) : n),
        month: (n, past) => (n === 1 ? (past ? t("vueUse.timeAgo.lastMonth") : t("vueUse.timeAgo.nextMonth")) : `${n} ${t(`vueUse.timeAgo.month`, n)}`),
        year: (n, past) => (n === 1 ? (past ? t("vueUse.timeAgo.lastYear") : t("vueUse.timeAgo.nextYear")) : `${n} ${t(`vueUse.timeAgo.year`, n)}`),
        day: (n, past) => (n === 1 ? (past ? t("vueUse.timeAgo.yesterday") : t("vueUse.timeAgo.tomorrow")) : `${n} ${t(`vueUse.timeAgo.day`, n)}`),
        week: (n, past) => (n === 1 ? (past ? t("vueUse.timeAgo.lastWeek") : t("vueUse.timeAgo.nextWeek")) : `${n} ${t(`vueUse.timeAgo.week`, n)}`),
        hour: (n) => `${n} ${t("vueUse.timeAgo.hour", n)}`,
        minute: (n) => `${n} ${t("vueUse.timeAgo.minute", n)}`,
        second: (n) => `${n} ${t(`vueUse.timeAgo.second`, n)}`,
        invalid: "INVALID DATE"
    };

    return useTimeAgo(date, {
        fullDateFormatter: (date: Date) => date.toLocaleDateString(),
        messages: I18N_MESSAGES
    });
}
