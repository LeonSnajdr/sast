import { VForm } from "vuetify/components";
import { required } from "@/rules";
import {
    __VLS_internalComponent,
    __VLS_componentsOption,
    __VLS_name,
    valid,
    form,
    taskSetName,
    taskSetDescription,
    createTaskSet
} from "./ProjectTaskSetCreate.vue";

function __VLS_template() {
    let __VLS_ctx!: InstanceType<__VLS_PickNotAny<typeof __VLS_internalComponent, new () => {}>> & {};
    /* Components */
    let __VLS_otherComponents!: NonNullable<typeof __VLS_internalComponent extends { components: infer C } ? C : {}> & typeof __VLS_componentsOption;
    let __VLS_own!: __VLS_SelfComponent<typeof __VLS_name, typeof __VLS_internalComponent & (new () => { $slots: typeof __VLS_slots })>;
    let __VLS_localComponents!: typeof __VLS_otherComponents & Omit<typeof __VLS_own, keyof typeof __VLS_otherComponents>;
    let __VLS_components!: typeof __VLS_localComponents & __VLS_GlobalComponents & typeof __VLS_ctx;
    /* Style Scoped */
    type __VLS_StyleScopedClasses = {};
    let __VLS_styleScopedClasses!: __VLS_StyleScopedClasses | keyof __VLS_StyleScopedClasses | (keyof __VLS_StyleScopedClasses)[];
    /* CSS variable injection */
    /* CSS variable injection end */
    let __VLS_resolvedLocalAndGlobalComponents!: {} & __VLS_WithComponent<
        "VRowSingle",
        typeof __VLS_localComponents,
        "VRowSingle",
        "vRowSingle",
        "v-row-single"
    > &
        __VLS_WithComponent<"VForm", typeof __VLS_localComponents, "VForm", "vForm", "v-form"> &
        __VLS_WithComponent<"VRow", typeof __VLS_localComponents, "VRow", "vRow", "v-row"> &
        __VLS_WithComponent<"VCol", typeof __VLS_localComponents, "VCol", "vCol", "v-col"> &
        __VLS_WithComponent<"VTextField", typeof __VLS_localComponents, "VTextField", "vTextField", "v-text-field"> &
        __VLS_WithComponent<"VBtnIcon", typeof __VLS_localComponents, "VBtnIcon", "vBtnIcon", "v-btn-icon">;
    __VLS_components.VRowSingle;
    __VLS_components.VRowSingle;
    __VLS_components.vRowSingle;
    __VLS_components.vRowSingle;
    __VLS_components["v-row-single"];
    __VLS_components["v-row-single"];
    // @ts-ignore
    [VRowSingle, VRowSingle];
    __VLS_components.VForm;
    __VLS_components.VForm;
    __VLS_components.vForm;
    __VLS_components.vForm;
    __VLS_components["v-form"];
    __VLS_components["v-form"];
    // @ts-ignore
    [VForm, VForm];
    __VLS_components.VRow;
    __VLS_components.VRow;
    __VLS_components.vRow;
    __VLS_components.vRow;
    __VLS_components["v-row"];
    __VLS_components["v-row"];
    // @ts-ignore
    [VRow, VRow];
    __VLS_components.VCol;
    __VLS_components.VCol;
    __VLS_components.VCol;
    __VLS_components.VCol;
    __VLS_components.vCol;
    __VLS_components.vCol;
    __VLS_components.vCol;
    __VLS_components.vCol;
    __VLS_components["v-col"];
    __VLS_components["v-col"];
    __VLS_components["v-col"];
    __VLS_components["v-col"];
    // @ts-ignore
    [VCol, VCol, VCol, VCol];
    __VLS_components.VTextField;
    __VLS_components.VTextField;
    __VLS_components.VTextField;
    __VLS_components.VTextField;
    __VLS_components.vTextField;
    __VLS_components.vTextField;
    __VLS_components.vTextField;
    __VLS_components.vTextField;
    __VLS_components["v-text-field"];
    __VLS_components["v-text-field"];
    __VLS_components["v-text-field"];
    __VLS_components["v-text-field"];
    // @ts-ignore
    [VTextField, VTextField, VTextField, VTextField];
    (({}) as __VLS_IntrinsicElements).template;
    (({}) as __VLS_IntrinsicElements).template;
    __VLS_components.VBtnIcon;
    __VLS_components.vBtnIcon;
    __VLS_components["v-btn-icon"];
    // @ts-ignore
    [VBtnIcon];
    {
        let __VLS_0!: "VRowSingle" extends keyof typeof __VLS_ctx
            ? typeof __VLS_ctx.VRowSingle
            : "vRowSingle" extends keyof typeof __VLS_ctx
            ? typeof __VLS_ctx.vRowSingle
            : "v-row-single" extends keyof typeof __VLS_ctx
            ? (typeof __VLS_ctx)["v-row-single"]
            : (typeof __VLS_resolvedLocalAndGlobalComponents)["VRowSingle"];
        const __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, new __VLS_0({ ...{} }));
        (({}) as { VRowSingle: typeof __VLS_0 }).VRowSingle;
        (({}) as { VRowSingle: typeof __VLS_0 }).VRowSingle;
        const __VLS_2 = __VLS_1({ ...{} }, ...__VLS_functionalComponentArgsRest(__VLS_1));
        (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_0, typeof __VLS_2> & Record<string, unknown>) => void)({ ...{} });
        const __VLS_3 = __VLS_pickFunctionalComponentCtx(__VLS_0, __VLS_2)!;
        let __VLS_4!: __VLS_NormalizeEmits<typeof __VLS_3.emit>;
        {
            let __VLS_5!: "VForm" extends keyof typeof __VLS_ctx
                ? typeof __VLS_ctx.VForm
                : "vForm" extends keyof typeof __VLS_ctx
                ? typeof __VLS_ctx.vForm
                : "v-form" extends keyof typeof __VLS_ctx
                ? (typeof __VLS_ctx)["v-form"]
                : (typeof __VLS_resolvedLocalAndGlobalComponents)["VForm"];
            const __VLS_6 = __VLS_asFunctionalComponent(__VLS_5, new __VLS_5({ ...{}, modelValue: __VLS_ctx.valid, ref: "form", class: "d-flex" }));
            (({}) as { VForm: typeof __VLS_5 }).VForm;
            (({}) as { VForm: typeof __VLS_5 }).VForm;
            const __VLS_7 = __VLS_6({ ...{}, modelValue: __VLS_ctx.valid, ref: "form", class: "d-flex" }, ...__VLS_functionalComponentArgsRest(__VLS_6));
            (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_5, typeof __VLS_7> & Record<string, unknown>) => void)({
                ...{},
                modelValue: __VLS_ctx.valid,
                ref: "form",
                class: "d-flex"
            });
            const __VLS_8 = __VLS_pickFunctionalComponentCtx(__VLS_5, __VLS_7)!;
            let __VLS_9!: __VLS_NormalizeEmits<typeof __VLS_8.emit>;
            // @ts-ignore
            __VLS_ctx.form;
            {
                let __VLS_10!: "VRow" extends keyof typeof __VLS_ctx
                    ? typeof __VLS_ctx.VRow
                    : "vRow" extends keyof typeof __VLS_ctx
                    ? typeof __VLS_ctx.vRow
                    : "v-row" extends keyof typeof __VLS_ctx
                    ? (typeof __VLS_ctx)["v-row"]
                    : (typeof __VLS_resolvedLocalAndGlobalComponents)["VRow"];
                const __VLS_11 = __VLS_asFunctionalComponent(__VLS_10, new __VLS_10({ ...{} }));
                (({}) as { VRow: typeof __VLS_10 }).VRow;
                (({}) as { VRow: typeof __VLS_10 }).VRow;
                const __VLS_12 = __VLS_11({ ...{} }, ...__VLS_functionalComponentArgsRest(__VLS_11));
                (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_10, typeof __VLS_12> & Record<string, unknown>) => void)({ ...{} });
                const __VLS_13 = __VLS_pickFunctionalComponentCtx(__VLS_10, __VLS_12)!;
                let __VLS_14!: __VLS_NormalizeEmits<typeof __VLS_13.emit>;
                {
                    let __VLS_15!: "VCol" extends keyof typeof __VLS_ctx
                        ? typeof __VLS_ctx.VCol
                        : "vCol" extends keyof typeof __VLS_ctx
                        ? typeof __VLS_ctx.vCol
                        : "v-col" extends keyof typeof __VLS_ctx
                        ? (typeof __VLS_ctx)["v-col"]
                        : (typeof __VLS_resolvedLocalAndGlobalComponents)["VCol"];
                    const __VLS_16 = __VLS_asFunctionalComponent(__VLS_15, new __VLS_15({ ...{} }));
                    (({}) as { VCol: typeof __VLS_15 }).VCol;
                    (({}) as { VCol: typeof __VLS_15 }).VCol;
                    const __VLS_17 = __VLS_16({ ...{} }, ...__VLS_functionalComponentArgsRest(__VLS_16));
                    (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_15, typeof __VLS_17> & Record<string, unknown>) => void)({ ...{} });
                    const __VLS_18 = __VLS_pickFunctionalComponentCtx(__VLS_15, __VLS_17)!;
                    let __VLS_19!: __VLS_NormalizeEmits<typeof __VLS_18.emit>;
                    {
                        let __VLS_20!: "VTextField" extends keyof typeof __VLS_ctx
                            ? typeof __VLS_ctx.VTextField
                            : "vTextField" extends keyof typeof __VLS_ctx
                            ? typeof __VLS_ctx.vTextField
                            : "v-text-field" extends keyof typeof __VLS_ctx
                            ? (typeof __VLS_ctx)["v-text-field"]
                            : (typeof __VLS_resolvedLocalAndGlobalComponents)["VTextField"];
                        const __VLS_21 = __VLS_asFunctionalComponent(
                            __VLS_20,
                            new __VLS_20({
                                ...{},
                                modelValue: __VLS_ctx.taskSetName,
                                placeholder: __VLS_ctx.$t("projectTaskSetCreate.input.name"),
                                rules: [
                                    __VLS_ctx.required(__VLS_ctx.$t("projectTaskSetCreate.input.name.required")),
                                    __VLS_ctx.unique(
                                        __VLS_ctx.project.placeholders.map((p) => p.name),
                                        __VLS_ctx.$t("projectPlaceholderCreate.input.name.notUnique")
                                    )
                                ]
                            })
                        );
                        (({}) as { VTextField: typeof __VLS_20 }).VTextField;
                        (({}) as { VTextField: typeof __VLS_20 }).VTextField;
                        const __VLS_22 = __VLS_21(
                            {
                                ...{},
                                modelValue: __VLS_ctx.taskSetName,
                                placeholder: __VLS_ctx.$t("projectTaskSetCreate.input.name"),
                                rules: [
                                    __VLS_ctx.required(__VLS_ctx.$t("projectTaskSetCreate.input.name.required")),
                                    __VLS_ctx.unique(
                                        __VLS_ctx.project.placeholders.map((p) => p.name),
                                        __VLS_ctx.$t("projectPlaceholderCreate.input.name.notUnique")
                                    )
                                ]
                            },
                            ...__VLS_functionalComponentArgsRest(__VLS_21)
                        );
                        (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_20, typeof __VLS_22> & Record<string, unknown>) => void)({
                            ...{},
                            modelValue: __VLS_ctx.taskSetName,
                            placeholder: __VLS_ctx.$t("projectTaskSetCreate.input.name"),
                            rules: [
                                __VLS_ctx.required(__VLS_ctx.$t("projectTaskSetCreate.input.name.required")),
                                __VLS_ctx.unique(
                                    __VLS_ctx.project.placeholders.map((p) => p.name),
                                    __VLS_ctx.$t("projectPlaceholderCreate.input.name.notUnique")
                                )
                            ]
                        });
                        const __VLS_23 = __VLS_pickFunctionalComponentCtx(__VLS_20, __VLS_22)!;
                        let __VLS_24!: __VLS_NormalizeEmits<typeof __VLS_23.emit>;
                    }
                    __VLS_18.slots!.default;
                }
                {
                    let __VLS_25!: "VCol" extends keyof typeof __VLS_ctx
                        ? typeof __VLS_ctx.VCol
                        : "vCol" extends keyof typeof __VLS_ctx
                        ? typeof __VLS_ctx.vCol
                        : "v-col" extends keyof typeof __VLS_ctx
                        ? (typeof __VLS_ctx)["v-col"]
                        : (typeof __VLS_resolvedLocalAndGlobalComponents)["VCol"];
                    const __VLS_26 = __VLS_asFunctionalComponent(__VLS_25, new __VLS_25({ ...{} }));
                    (({}) as { VCol: typeof __VLS_25 }).VCol;
                    (({}) as { VCol: typeof __VLS_25 }).VCol;
                    const __VLS_27 = __VLS_26({ ...{} }, ...__VLS_functionalComponentArgsRest(__VLS_26));
                    (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_25, typeof __VLS_27> & Record<string, unknown>) => void)({ ...{} });
                    const __VLS_28 = __VLS_pickFunctionalComponentCtx(__VLS_25, __VLS_27)!;
                    let __VLS_29!: __VLS_NormalizeEmits<typeof __VLS_28.emit>;
                    {
                        let __VLS_30!: "VTextField" extends keyof typeof __VLS_ctx
                            ? typeof __VLS_ctx.VTextField
                            : "vTextField" extends keyof typeof __VLS_ctx
                            ? typeof __VLS_ctx.vTextField
                            : "v-text-field" extends keyof typeof __VLS_ctx
                            ? (typeof __VLS_ctx)["v-text-field"]
                            : (typeof __VLS_resolvedLocalAndGlobalComponents)["VTextField"];
                        const __VLS_31 = __VLS_asFunctionalComponent(
                            __VLS_30,
                            new __VLS_30({
                                ...{},
                                modelValue: __VLS_ctx.taskSetDescription,
                                placeholder: __VLS_ctx.$t("projectTaskSetCreate.input.description")
                            })
                        );
                        (({}) as { VTextField: typeof __VLS_30 }).VTextField;
                        (({}) as { VTextField: typeof __VLS_30 }).VTextField;
                        const __VLS_32 = __VLS_31(
                            { ...{}, modelValue: __VLS_ctx.taskSetDescription, placeholder: __VLS_ctx.$t("projectTaskSetCreate.input.description") },
                            ...__VLS_functionalComponentArgsRest(__VLS_31)
                        );
                        (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_30, typeof __VLS_32> & Record<string, unknown>) => void)({
                            ...{},
                            modelValue: __VLS_ctx.taskSetDescription,
                            placeholder: __VLS_ctx.$t("projectTaskSetCreate.input.description")
                        });
                        const __VLS_33 = __VLS_pickFunctionalComponentCtx(__VLS_30, __VLS_32)!;
                        let __VLS_34!: __VLS_NormalizeEmits<typeof __VLS_33.emit>;
                        {
                            const __VLS_35 = ({} as __VLS_IntrinsicElements)["template"];
                            const __VLS_36 = __VLS_elementAsFunctionalComponent(__VLS_35);
                            (({}) as __VLS_IntrinsicElements).template;
                            (({}) as __VLS_IntrinsicElements).template;
                            const __VLS_37 = __VLS_36({ ...{} }, ...__VLS_functionalComponentArgsRest(__VLS_36));
                            (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_35, typeof __VLS_37> & Record<string, unknown>) => void)({ ...{} });
                            {
                                __VLS_33.slots!.append;
                                {
                                    let __VLS_38!: "VBtnIcon" extends keyof typeof __VLS_ctx
                                        ? typeof __VLS_ctx.VBtnIcon
                                        : "vBtnIcon" extends keyof typeof __VLS_ctx
                                        ? typeof __VLS_ctx.vBtnIcon
                                        : "v-btn-icon" extends keyof typeof __VLS_ctx
                                        ? (typeof __VLS_ctx)["v-btn-icon"]
                                        : (typeof __VLS_resolvedLocalAndGlobalComponents)["VBtnIcon"];
                                    const __VLS_39 = __VLS_asFunctionalComponent(__VLS_38, new __VLS_38({ ...{ onClick: {} as any }, icon: "mdi-plus" }));
                                    (({}) as { VBtnIcon: typeof __VLS_38 }).VBtnIcon;
                                    const __VLS_40 = __VLS_39({ ...{ onClick: {} as any }, icon: "mdi-plus" }, ...__VLS_functionalComponentArgsRest(__VLS_39));
                                    (({}) as (props: __VLS_FunctionalComponentProps<typeof __VLS_38, typeof __VLS_40> & Record<string, unknown>) => void)({
                                        ...{ onClick: {} as any },
                                        icon: "mdi-plus"
                                    });
                                    const __VLS_41 = __VLS_pickFunctionalComponentCtx(__VLS_38, __VLS_40)!;
                                    let __VLS_42!: __VLS_NormalizeEmits<typeof __VLS_41.emit>;
                                    let __VLS_43 = {
                                        click: __VLS_pickEvent(
                                            __VLS_42["click"],
                                            ({} as __VLS_FunctionalComponentProps<typeof __VLS_39, typeof __VLS_40>).onClick
                                        )
                                    };
                                    __VLS_43 = {
                                        click: __VLS_ctx.createTaskSet
                                    };
                                }
                            }
                        }
                    }
                    __VLS_28.slots!.default;
                }
                __VLS_13.slots!.default;
            }
            __VLS_8.slots!.default;
        }
        __VLS_3.slots!.default;
    }
    if (typeof __VLS_styleScopedClasses === "object" && !Array.isArray(__VLS_styleScopedClasses)) {
    }
    var __VLS_slots!: {};
    // @ts-ignore
    [
        valid,
        valid,
        valid,
        form,
        taskSetName,
        $t,
        required,
        $t,
        unique,
        project,
        $t,
        taskSetName,
        $t,
        required,
        $t,
        unique,
        project,
        $t,
        taskSetName,
        $t,
        required,
        $t,
        unique,
        project,
        $t,
        taskSetDescription,
        $t,
        taskSetDescription,
        $t,
        taskSetDescription,
        $t,
        createTaskSet
    ];
    return __VLS_slots;
}
