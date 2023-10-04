/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}", "./node_modules/primevue/**/*.{vue,js,ts,jsx,tsx}"],
    theme: {
        extend: {
            fontFamily: {
                nunito: ["Nunito", "sans-serif"]
            }
        },
        colors: {
            primary: "#303A4E",
            "primary-lighten-1": "#636B7A",
            "primary-lighten-2": "#969BA5",
            "primary-lighten-3": "#E4E5E8",
            secondary: "#8098A7",
            "secondary-lighten-1": "#9FB1BD",
            "secondary-lighten-2": "#BFCBD2",
            "secondary-lighten-3": "#DEE4E8",
            "secondary-lighten-4": "#F2F5F6",
            tertiary: "#599B8B",
            "tertiary-lighten-1": "#82B4A8",
            "tertiary-lighten-2": "#ABCCC4",
            "tertiary-lighten-3": "#D4E5E1",
            background: "#F9FAFC",
            surface: "#FFFFFF",
            "on-surface": "#1C1C1C",
            error: "#E52727",
            "error-lighten-1": "#DCBABA",
            warning: "#DC8B22",
            "warning-lighten-1": "#E2D4C1",
            success: "#7D9E39",
            "success-lighten-1": "#D7E1C2",
            info: "#2196F3"
        }
    },
    plugins: []
};
