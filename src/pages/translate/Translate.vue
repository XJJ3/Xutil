<template>
  <div class="translate_wrapper" data-tauri-drag-region>
    <n-input
      v-model:value="inputVal"
      placeholder="Shift + Enter 换行"
      type="textarea"
      :autosize="{
        minRows: 1,
        maxRows: 3,
      }"
      @keydown="handleKeydown"
    >
      <template #prefix>
        <svg
          v-show="fromLang === 'zh'"
          style="width: 16px; height: 16px; cursor: pointer; margin-right: 2px"
          class="icon"
          viewBox="0 0 1024 1024"
          width="48"
          height="48"
          @click="() => handleChangeFromLang('en')"
        >
          <path
            d="M160 144a32 32 0 0 0-32 32V864a32 32 0 0 0 32 32h688a32 32 0 0 0 32-32V176a32 32 0 0 0-32-32H160z m0-64h688a96 96 0 0 1 96 96V864a96 96 0 0 1-96 96H160a96 96 0 0 1-96-96V176a96 96 0 0 1 96-96z"
            fill="#cdcdcd"
          ></path>
          <path
            d="M482.176 262.272h59.616v94.4h196v239.072h-196v184.416h-59.616v-184.416H286.72v-239.04h195.456V262.24z m-137.504 277.152h137.504v-126.4H344.64v126.4z m197.12 0h138.048v-126.4H541.76v126.4z"
            fill="#cdcdcd"
          ></path>
        </svg>
        <svg
          v-show="fromLang === 'en'"
          style="width: 16px; height: 16px; cursor: pointer; margin-right: 2px"
          class="icon"
          viewBox="0 0 1024 1024"
          width="48"
          height="48"
          @click="() => handleChangeFromLang('zh')"
        >
          <path
            d="M229.248 704V337.504h271.744v61.984h-197.76v81.28h184v61.76h-184v99.712h204.768V704h-278.72z m550.496 0h-70.24v-135.488c0-28.672-1.504-47.232-4.48-55.648a39.04 39.04 0 0 0-14.656-19.616 41.792 41.792 0 0 0-24.384-7.008c-12.16 0-23.04 3.328-32.736 10.016-9.664 6.656-16.32 15.488-19.872 26.496-3.584 11.008-5.376 31.36-5.376 60.992V704h-70.24v-265.504h65.248v39.008c23.168-30.016 52.32-44.992 87.488-44.992 15.52 0 29.664 2.784 42.496 8.352 12.832 5.6 22.56 12.704 29.12 21.376 6.592 8.672 11.2 18.496 13.76 29.504 2.56 11.008 3.872 26.752 3.872 47.264V704z"
            fill="#cdcdcd"
          ></path>
          <path
            d="M160 144a32 32 0 0 0-32 32V864a32 32 0 0 0 32 32h688a32 32 0 0 0 32-32V176a32 32 0 0 0-32-32H160z m0-64h688a96 96 0 0 1 96 96V864a96 96 0 0 1-96 96H160a96 96 0 0 1-96-96V176a96 96 0 0 1 96-96z"
            fill="#cdcdcd"
          ></path>
        </svg>
      </template>
      <template #suffix>
        <svg
          style="width: 20px; height: 20px; cursor: pointer"
          class="icon"
          viewBox="0 0 1024 1024"
          width="48"
          height="48"
          @click="handleClick"
        >
          <path
            d="M929.299 739.872L774.98 634.816c31.086-51.044 48.995-110.994 48.995-175.129 0-186.22-150.96-337.181-337.18-337.181s-337.18 150.962-337.18 337.182c0 186.218 150.961 337.179 337.181 337.179 101.842 0 193.139-45.151 254.964-116.527l155.843 106.097a28.021 28.021 0 0 0 15.823 4.886c9.006 0 17.854-4.312 23.306-12.316 8.752-12.86 5.424-30.377-7.433-39.135zM596.1 718.493c-34.589 14.629-71.364 22.047-109.304 22.047s-74.716-7.418-109.305-22.047c-33.437-14.142-63.478-34.4-89.29-60.213-25.811-25.811-46.068-55.852-60.213-89.29-14.63-34.589-22.047-71.368-22.047-109.303 0-37.941 7.417-74.715 22.047-109.304 14.145-33.438 34.402-63.48 60.213-89.29 25.813-25.813 55.854-46.07 89.29-60.214 34.589-14.63 71.365-22.049 109.305-22.049s74.715 7.419 109.304 22.049c33.437 14.144 63.479 34.4 89.291 60.214 25.81 25.81 46.069 55.852 60.212 89.29 14.63 34.589 22.047 71.362 22.047 109.304 0 37.935-7.417 74.714-22.047 109.303a279.709 279.709 0 0 1-26.677 48.872 28.078 28.078 0 0 0-7.705 7.616 28.142 28.142 0 0 0-3.768 8 284.009 284.009 0 0 1-22.063 24.802c-25.812 25.813-55.854 46.072-89.29 60.213z"
            fill="#464899"
            p-id="7799"
          ></path>
          <path
            d="M661.921 328.785a22.414 22.414 0 0 1-11.787-3.346 179.688 179.688 0 0 0-23.571-12.198c-11.446-4.884-16.764-18.119-11.881-29.563 4.884-11.446 18.12-16.765 29.563-11.881a224.827 224.827 0 0 1 29.505 15.268c10.598 6.522 13.9 20.4 7.38 30.996-4.252 6.915-11.646 10.724-19.209 10.724M317.785 352.971c-4.808 0-9.649-1.53-13.749-4.694-9.852-7.603-11.675-21.75-4.071-31.602 33.717-43.69 82.21-73.227 136.543-83.166 43.232-7.907 87.37-3.009 127.637 14.177 11.446 4.884 16.764 18.12 11.881 29.564-4.884 11.445-18.12 16.765-29.564 11.88-32.114-13.703-67.331-17.608-101.846-11.299-43.365 7.937-82.07 31.507-108.978 66.373-4.441 5.752-11.111 8.767-17.853 8.767"
            fill="#E84C57"
            p-id="7800"
          ></path>
        </svg>
      </template>
    </n-input>

    <n-input
      :value="transVal"
      placeholder=""
      type="textarea"
      readonly
      style="margin-top: 10px"
      :autosize="{
        minRows: 3,
        maxRows: 3,
      }"
    ></n-input>
  </div>
</template>
<script setup lang="ts">
import { encryptByMd5 } from '@/utils/encrypt';
import { invoke } from '@tauri-apps/api';
import { appLogDir } from '@tauri-apps/api/path';

const message = useMessage();
const inputVal = ref('');
const transVal = ref('');
const fromLang = ref<'zh' | 'en'>('zh');

const handleClick = () => {
  // console.log('!23231');
  // translateReq();
  translateReq();
};

const handleChangeFromLang = (from: 'zh' | 'en') => {
  fromLang.value = from;
  message.destroyAll();
  message.info(`切换到${from === 'zh' ? '中文' : '英文'}转${from === 'zh' ? '英文' : '中文'}`);
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Enter') {
    if (!event.shiftKey && !event.isComposing) {
      translateReq();
      event.preventDefault(); // 阻止默认的回车键行为
      event.stopPropagation(); // 阻止事件继续传播
    }
  }
};

const translateReq = () => {
  const content = inputVal.value;

  if (!content) {
    transVal.value = '';
    return;
  }

  const url = 'https://fanyi-api.baidu.com/api/trans/vip/translate';
  const appid = '20240417002026646';
  const key = 'u7UPXSZUzozjxwDPI1Rv';
  const slatStr = genSalt();

  // const str1 = `${appid}${content}${slatStr}${key}`;
  const str1 = appid + content + slatStr + key;
  const sign = encryptByMd5(str1);

  const from = fromLang.value;
  const to = fromLang.value === 'zh' ? 'en' : 'zh';

  const reqUrl = `${url}?q=${encodeURI(
    content
  )}&from=${from}&to=${to}&appid=${appid}&salt=${slatStr}&sign=${sign}`;

  invoke('dispatch_command', {
    name: 'translate',
    args: {
      url: reqUrl,
    },
  }).then((response: any) => {
    const res = JSON.parse(response.body);
    let content = '';
    for (let i = 0; i < res.trans_result.length; i++) {
      content += res.trans_result?.[i]?.dst;
      content += '\n';
    }
    transVal.value = content;
  });
};

function genSalt() {
  return Math.random().toString(36).substring(2, 10);
}

onMounted(() => {
  appLogDir().then((res) => {
    console.log(res);
  });
});
</script>
<style scoped lang="scss">
.translate_wrapper {
  width: 100%;
  height: 100%;
  padding-top: 10px;
}
</style>
