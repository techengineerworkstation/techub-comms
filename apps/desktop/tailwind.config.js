/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
    '../../packages/shared/src/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        beige: {
          50: '#fdf8f0',
          100: '#ecdcb8',
          200: '#d9c49a',
          300: '#c5ab7b',
          400: '#b1935d',
          500: '#9d7a3e',
        },
        teal: {
          50: '#e6f7f7',
          100: '#b3e8e8',
          200: '#80d9d9',
          300: '#4dcaca',
          400: '#1abbbb',
          500: '#009999',
          600: '#007a7a',
          700: '#005c5c',
        },
      },
    },
  },
  plugins: [],
};
