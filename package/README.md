# @ohos-rs/mmkv

Native binding package for mmkv.

## ！This is an early preview version, providing only a small number of APIs.

## Install

```shell
ohpm install @ohos-rs/mmkv
```

## Usage

```ts
import { MMKV } from '@ohos-rs/mmkv';

const m = new MMKV("/data/storage/el2/base/haps/entry/files/mmkv",MMKVLogLevel.Info,MMKVMode.SingleProcess);

m.encodeBool("test",false,16000);
const a = m.decodeBool("test");
```