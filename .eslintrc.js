module.exports = {
  root: true,
  env: {
    node: true,
  },
  extends: [
    'plugin:vue/vue3-recommended',
    '@vue/typescript/recommended',
    'plugin:security/recommended',
    'prettier',
    'plugin:prettier/recommended',
    './.eslintrc-auto-import.json',
  ],
  parserOptions: {
    ecmaVersion: 2021,
  },
  plugins: ['simple-import-sort'],
  rules: {
    'no-var': 'error',
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'comma-dangle': ['error', 'only-multiline'],
    'vue/multi-word-component-names': 'off',
    'no-shadow': 0, //  关闭全局的检查，采用 ts 的，避免 enum 的报错
    '@typescript-eslint/no-shadow': 2,
    'no-unused-vars': 0, // 关闭全局的检查，采用 ts 的，规避 enum 的报错
    '@typescript-eslint/no-unused-vars': 2,
    'vue/comment-directive': 'off',
    // 'max-len': ['warn', { code: 150 }],
    'simple-import-sort/imports': [
      'error',
      {
        groups: [
          // Side effect imports.
          ['^\\u0000'],
          // Packages.
          // Things that start with a letter (or digit or underscore), or `@` followed by a letter.
          ['^@?\\w'],
          // Absolute imports and other imports such as Vue-style `@/foo`.
          // Anything not matched in another group.
          ['^'],
          // Relative imports.
          // Anything that starts with a dot.
          ['^\\.'],
          // 样式放最后
          ['.*\\.(s|(post))?css$'],
        ],
      },
    ],
  },
  globals: {
    defineProps: 'readonly',
    defineEmits: 'readonly',
    defineExpose: 'readonly',
    withDefaults: 'readonly',
    __LOCAL__: 'readonly',
  },
};
