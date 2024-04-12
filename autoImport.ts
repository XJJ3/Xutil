import AutoImport from 'unplugin-auto-import/vite';

export function autoImportConfig() {
  return AutoImport({
    include: [
      /\.tsx?$/, // .ts, .tsx
      /\.vue$/,
      /\.vue\?vue/, // .vue
    ],
    imports: [
      'vue',
      'vue-router',
      'pinia',
      {
        '@/store/index': ['useAdminMainStore', 'useAdminUserStore'],
      },
      {
        '@vueuse/core': ['useToggle'],
      },
      {
        '@/admin/echarts': ['echarts'],
      },
    ],
    dts: 'types/auto-imports.d.ts',
    eslintrc: {
      enabled: true,
      filepath: '/.eslintrc-auto-import.json',
    },
  });
}
