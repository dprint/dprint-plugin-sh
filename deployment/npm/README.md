# @dprint/shuck

npm distribution of [dprint-plugin-shuck](https://github.com/dprint/dprint-plugin-shuck) which is an adapter plugin for [shuck-formatter](https://github.com/ewhauser/shuck).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-shuck Wasm file](https://github.com/dprint/dprint-plugin-shuck/releases).

## Example

```ts
import { createFromBuffer } from "@dprint/formatter";
import { getBuffer } from "@dprint/shuck";

const formatter = createFromBuffer(getBuffer());

console.log(formatter.formatText({
  filePath: "file.sh",
  fileText: "echo   hello\n",
}));
```
