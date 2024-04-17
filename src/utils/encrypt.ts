// from vben
import CryptoJS from 'crypto-js';
import { decrypt, encrypt } from 'crypto-js/aes';
import Base64 from 'crypto-js/enc-base64';
import { parse } from 'crypto-js/enc-utf8';
import UTF8 from 'crypto-js/enc-utf8';
import MD5 from 'crypto-js/md5';
import ECB from 'crypto-js/mode-ecb';
import pkcs7 from 'crypto-js/pad-pkcs7';

export interface EncryptionParams {
  key: string;
  iv: string;
}

export class AesEncryption {
  private key: CryptoJS.lib.WordArray = parse('');
  private iv: CryptoJS.lib.WordArray = parse('');

  constructor(opt: Partial<EncryptionParams> = {}) {
    const { key, iv } = opt;
    if (key) {
      this.key = parse(key);
    }
    if (iv) {
      this.iv = parse(iv);
    }
  }

  get getOptions() {
    return {
      mode: ECB,
      padding: pkcs7,
      iv: this.iv,
    };
  }

  encryptByAES(cipherText: string) {
    return encrypt(cipherText, this.key, this.getOptions).toString();
  }

  decryptByAES(cipherText: string) {
    return decrypt(cipherText, this.key, this.getOptions).toString(UTF8);
  }
}

export function encryptByBase64(cipherText: string) {
  return UTF8.parse(cipherText).toString(Base64);
}

export function decodeByBase64(cipherText: string) {
  return Base64.parse(cipherText).toString(UTF8);
}

export function encryptByMd5(password: string) {
  return MD5(password).toString();
}

/**
 * @description 获取文件的 MD5 值
 * @param {Blob || File} file 待处理文件(可以为 input 上传的文件或者 blob 对象)
 * @return Promise<string>
 */
export function fileMd5Sum(file: File | Blob) {
  return new Promise((resolve, reject) => {
    try {
      const fileReader = new FileReader();
      fileReader.onloadend = (ev) => {
        resolve(
          CryptoJS.MD5(CryptoJS.enc.Latin1.parse(ev.target?.result as string)).toString(
            CryptoJS.enc.Hex
          )
        );
      };
      fileReader.readAsBinaryString(file);
    } catch (e) {
      reject(e);
    }
  });
}
