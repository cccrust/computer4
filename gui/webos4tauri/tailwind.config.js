/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        os: {
          bg: '#0a0a0f',
          surface: '#14141f',
          card: '#1c1c2e',
          border: '#2a2a3e',
          text: '#e8e8f0',
          'text-secondary': '#9090a8',
          accent: '#5b5bf0',
          'accent-hover': '#6f6ff5',
          danger: '#f05b5b',
        }
      },
      fontFamily: {
        sans: ['-apple-system', 'BlinkMacSystemFont', 'SF Pro Display', 'SF Pro Text', 'system-ui', 'sans-serif'],
        mono: ['SF Mono', 'SFMono-Regular', 'ui-monospace', 'monospace'],
      },
      spacing: {
        'safe-top': 'env(safe-area-inset-top, 0px)',
        'safe-bottom': 'env(safe-area-inset-bottom, 0px)',
      },
      borderRadius: {
        'app': '12px',
        'icon': '20px',
      },
      width: {
        'app-icon': '64px',
      },
      height: {
        'app-icon': '64px',
      },
    },
  },
  plugins: [],
}
