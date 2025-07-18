import type { ValidationRule } from "vuetify";

export function required(message: string): ValidationRule {
    return (value: string) => {
        return (value && lodTrim(value).length > 0) || message;
    };
}

export function json(message: string): ValidationRule {
    return (value: string) => {
        try {
            JSON.parse(value);
            return true;
        } catch {
            return message;
        }
    };
}

export function email(message: string): ValidationRule {
    const emailPattern = /^[A-Z0-9+_.-]+@[A-Z0-9.-]+$/i;
    return regex(emailPattern, message);
}

export function regex(regex: RegExp, message: string): ValidationRule {
    return (value: string) => {
        return regex.test(value) || message;
    };
}
export function exists(message: string, items: string[]): ValidationRule {
    return (value: string) => {
        // operator == is required to ensure it also works for ref<string>
        const valid = value && items && lodSome(items, (i) => i == value);
        return valid || message;
    };
}

export function notExists(message: string, items: string[]): ValidationRule {
    return (value: string) => {
        // operator == is required to ensure it also works for ref<string>
        const valid = value && items && !lodSome(items, (i) => i == value);
        return valid || message;
    };
}

export function hasItems(message: string): ValidationRule {
    return (value: unknown[]) => {
        const valid = value.length > 0;
        return valid || message;
    };
}

export function validName(message: string): ValidationRule {
    const namePattern = /^(?!.* {2})(?:[0-9a-zA-ZÄÖÜäöüß\-_ ]+)?$/;
    return regex(namePattern, message);
}
