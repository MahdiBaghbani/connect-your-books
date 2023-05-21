// All this does though is instruct Tailwind to look at all Rust, HTML, and CSS files in src/ and remove any classes it
// doesn't find being used there. We also check index.html in case you have anything in there.

module.exports = {
    // add the folders and files from your templates that might have css classes in them.
    content: [
        "./index.html",
        "./src/**/*.rs",
        "./dist/**/*.html",
        "./dist/**/*.css",
    ],
    // enable dark mode via class strategy.
    darkMode: 'class',
    theme: {
        extend: {
            // extend base Tailwind CSS utility classes.
            colors: {
                primary: {
                    "50": "#eff6ff",
                    "100": "#dbeafe",
                    "200": "#bfdbfe",
                    "300": "#93c5fd",
                    "400": "#60a5fa",
                    "500": "#3b82f6",
                    "600": "#2563eb",
                    "700": "#1d4ed8",
                    "800": "#1e40af",
                    "900": "#1e3a8a"
                }
            }
        },
        fontFamily: {
            'body': [
                'Inter',
                'ui-sans-serif',
                'system-ui',
                '-apple-system',
                'system-ui',
                'Segoe UI',
                'Roboto',
                'Helvetica Neue',
                'Arial',
                'Noto Sans',
                'sans-serif',
                'Apple Color Emoji',
                'Segoe UI Emoji',
                'Segoe UI Symbol',
                'Noto Color Emoji'
            ],
            'sans': [
                'Inter',
                'ui-sans-serif',
                'system-ui',
                '-apple-system',
                'system-ui',
                'Segoe UI',
                'Roboto',
                'Helvetica Neue',
                'Arial',
                'Noto Sans',
                'sans-serif',
                'Apple Color Emoji',
                'Segoe UI Emoji',
                'Segoe UI Symbol',
                'Noto Color Emoji'
            ]
        }
    },
    variants: {},
    plugins: [
        require('@tailwindcss/forms'),
    ],
};
