/* eslint-disable @typescript-eslint/no-explicit-any */

declare module "vuetify/helpers" {
    // @see vuetify/lib/labs/components.d.mts
    type SortItem = {
        key: string;
        order?: boolean | "asc" | "desc";
    };

    type DataTableHeader = {
        key: string;
        value?: SelectItemKey;
        title: string;
        colspan?: number;
        rowspan?: number;
        fixed?: boolean;
        align?: "start" | "end" | "center";
        width?: number | string;
        minWidth?: string;
        maxWidth?: string;
        sortable?: boolean;
        sort?: DataTableCompareFunction;
    };

    interface ListItem {
        title: string;
        value: any;
    }

    type ValidationResult = string | boolean;
    type ValidationRule =
        | ValidationResult
        | PromiseLike<ValidationResult>
        | ((value: any) => ValidationResult)
        | ((value: any) => PromiseLike<ValidationResult>);

    export { SortItem, ListItem, DataTableHeader, ValidationResult, ValidationRule };
}
