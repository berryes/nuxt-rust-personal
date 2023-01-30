export default {
  ssr:true,
  // Global page headers: https://go.nuxtjs.dev/config-head
  head: {

    htmlAttrs: {
        class: 'no-js',
    },
    script: [
        {
            hid: 'no-js',
            type: 'text/javascript',
            innerHTML: `
                document.documentElement.classList.remove('no-js');
            `,
        },
    ],
    __dangerouslyDisableSanitizers: ['script'],
    changed() {
        if (typeof window !== 'undefined') {
            document.documentElement.classList.remove('no-js');
        }
    },
},

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: [
    "~fonts/JetBrainsMonoNL-Medium.ttf",
    '~/css/main.css'
  ],

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: [
  ],
  generate: { fallback: true },
  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [
  ],

  modules: [
    // https://go.nuxtjs.dev/axios
    '@nuxtjs/proxy'
  ],

  proxy: {
    '/api': {
      target: 'http://127.0.0.1:8000',
      changeOrigin: true,
      pathRewrite: { '^/api': '/' },
    },
  },

  // Build Configuration: https://go.nuxtjs.dev/config-build
  build: {
  }
}
