/* eslint-disable @typescript-eslint/no-explicit-any */

declare module "vuetify/helpers" {
    // @see vuetify/lib/components/index.d.mts
    type SortItem = {
        key: string;
        order?: boolean | "asc" | "desc";
    };

    type IconValue = string | (string | [path: string, opacity: number])[] | JSXComponent;

    type InternalDataTableHeader = Omit<DataTableHeader, "key" | "value" | "children"> & {
        key: string | null;
        value: SelectItemKey | null;
        sortable: boolean;
        fixedOffset?: number;
        lastFixed?: boolean;
        colspan?: number;
        rowspan?: number;
        children?: InternalDataTableHeader[];
    };

    type DataTableHeader = {
        key?: "data-table-group" | "data-table-select" | "data-table-expand" | (string & NonNullable<unknown>);
        value?: SelectItemKey;
        title?: string;
        fixed?: boolean;
        align?: "start" | "end" | "center";
        width?: number | string;
        minWidth?: string;
        maxWidth?: string;
        headerProps?: Record<string, any>;
        cellProps?: HeaderCellProps;
        sortable?: boolean;
        sort?: DataTableCompareFunction;
        filter?: FilterFunction;
        children?: DataTableHeader[];
    };

    interface InternalItem<T = any> {
        value: any;
        raw: T;
    }

    interface ListItem<T = any> extends InternalItem<T> {
        title: string;
        props: {
            [key: string]: any;
            title: string;
            value: any;
        };
        children?: ListItem<T>[];
    }

    type ValidationResult = string | boolean;
    type ValidationRule =
        | ValidationResult
        | PromiseLike<ValidationResult>
        | ((value: any) => ValidationResult)
        | ((value: any) => PromiseLike<ValidationResult>);

    export { SortItem, ListItem, DataTableHeader, ValidationResult, ValidationRule, InternalDataTableHeader, IconValue };

    export type Density = null | "default" | "comfortable" | "compact";
}
