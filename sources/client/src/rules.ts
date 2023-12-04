import { trim } from "lodash";
import type { ValidationRule } from "vuetify/helpers";

export function required(message: string): ValidationRule {
    return (value: string) => {
        return (value && trim(value).length > 0) || message;
    };
}

export function unique(valueList: string[], message: string): ValidationRule {
    return (value: string) => {
        return !valueList.includes(value) || message;
    };
}
