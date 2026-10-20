# @dprint/sh

npm distribution of [dprint-plugin-sh](https://github.com/dprint/dprint-plugin-sh) which is an adapter plugin for [shuck-formatter](https://github.com/ewhauser/shuck).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-sh Wasm file](https://github.com/dprint/dprint-plugin-sh/releases).

## Example

```ts
import { createFromBuffer } from "@dprint/formatter";
import { getBuffer } from "@dprint/sh";

const formatter = createFromBuffer(getBuffer());

console.log(formatter.formatText("file.sh", "echo   hello\n"));
```
