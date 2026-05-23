/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        beige: {
          50: '#fdf8f0',
          100: '#f5ead6',
          200: '#ecdcb8',
          300: '#dfc89a',
          400: '#d4b57e',
          500: '#c4a06a',
          600: '#a68550',
          700: '#8a6d3e',
          800: '#6e5630',
          900: '#523f24',
        },
        teal: {
          50: '#e6f7f7',
          100: '#b3e8e8',
          200: '#80d9d9',
          300: '#4dcaca',
          400: '#26bfbf',
          500: '#009999',
          600: '#007a7a',
          700: '#005c5c',
          800: '#003d3d',
          900: '#001f1f',
        },
      },
      backgroundColor: {
        app: 'var(--color-bg)',
        surface: 'var(--color-surface)',
      },
      textColor: {
        primary: 'var(--color-primary)',
        muted: 'var(--color-text-muted)',
      },
    },
  },
  plugins: [],
};
