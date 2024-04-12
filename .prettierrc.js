module.exports = {
  printWidth: 100, // 一行的字符数，如果超过会进行换行，默认为80
  tabWidth: 2, // 一个tab代表几个空格数
  endOfLine: 'auto', // 换行符 Linux环境文件行尾序列是CRLF Windows是LF
  useTabs: false, // 是否使用tab进行缩进，默认为false，表示用空格进行缩减
  singleQuote: true, // 字符串是否使用单引号，默认为false，true-使用双引号
  semi: true, // 是否在行尾使用分号，默认为true，这里选择不加分号
  trailingComma: 'es5', // 是否使用尾逗号，有三个可选值"<none|es5|all>" es5 包括es5中的数组、对象 all 包括函数对象等所有可选 在对象或数组最后一个元素后面是否加逗号（在ES5中加尾逗号）
  bracketSpacing: true, // 对象大括号直接是否有空格，默认为true，效果：{ foo: bar }
  proseWrap: 'never',
  jsxBracketSameLine: true,
  jsxSingleQuote: false,
  htmlWhitespaceSensitivity: 'strict',
  overrides: [
    {
      files: '*.html',
      options: {
        parser: 'html',
      },
    },
  ],
};
