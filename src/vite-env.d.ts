/// <reference types="vite/client" />

declare module '*.vue' {
  import { DefineComponent } from 'vue';
  // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/ban-types
  const component: DefineComponent<AnyType, {}, AnyType>;
  export default component;
}

interface ImportMetaEnv extends Readonly<Record<string, string>> {
  // Only string type here to avoid hard to debug cast problems in your components!
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
