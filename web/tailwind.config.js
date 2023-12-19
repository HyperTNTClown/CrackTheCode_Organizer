/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,ts}",
  ],
  theme: {
    extend: {},
    listStyleType: {
      none: 'none',
      disc: 'disc',
      decimal: 'decimal',
      square: 'square',
      roman: 'upper-roman',
      alpha: 'upper-alpha',
    },
  },
  plugins: [require('daisyui')],
  daisyui: {
    themes: true,
    base: true,
    styled: true,
    utils: true,
    logs: true,
  }
}

