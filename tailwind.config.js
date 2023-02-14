module.exports = {
    content: [
        "./src/**/*.rs",
    ],
    theme: {
        extend: {
            colors: {
                'not-quite-black': '#23272a',
            }
        },
        screens: {
            'desktop': '1436px',
            'mobile': {max: '1435px'}
        },
        fontFamily: {
            brand: ['Inter', 'Roboto Mono']
        },
    },
    variants: {},
    plugins: [],
};