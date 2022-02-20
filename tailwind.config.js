module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./src/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    theme: {},
    variants: {},
    plugins: [require("@tailwindcss/typography")],
};
