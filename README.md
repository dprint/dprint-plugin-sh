# dprint-plugin-shuck

[![npm version](https://img.shields.io/npm/v/@dprint/shuck.svg)](https://www.npmjs.com/package/@dprint/shuck) [![CI](https://github.com/dprint/dprint-plugin-shuck/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-shuck/actions?query=workflow%3ACI)

Adapter for [shuck-formatter](https://crates.io/crates/shuck-formatter) for use as a formatting plugin in [dprint](https://github.com/dprint/dprint). It formats shell scripts.

Originally created by [Kenichi Kamiya](https://github.com/kachick).

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

Then in your project's directory with a dprint.json file, run:

```shellsession
dprint add shuck
```

Note: You do not need shfmt or shuck installed globally as dprint will run shuck-formatter from the .wasm file in a sandboxed environment.

## Configuration

Works with no configuration by default. To add configuration, specify a `"shuck"` key in your dprint.json:

```jsonc
{
  "shuck": {
    "dialect": "auto",
    "indentWidth": 8,
    "useTabs": true,
    "binaryNextLine": false,
    "switchCaseIndent": false,
    "spaceRedirects": false,
    "keepPadding": false,
    "functionNextLine": false,
    "neverSplit": false,
    "simplify": false,
    "minify": false,
  },
  "plugins": [
    // ...etc...
  ],
}
```

### Options

This plugin directly uses default options from the upstream `shuck-formatter` crate.
Current defaults are shown below:

| Option             | Type    | Default  | Description                                                                              |
| ------------------ | ------- | -------- | ---------------------------------------------------------------------------------------- |
| `dialect`          | string  | `"auto"` | Shell dialect: `"auto"`, `"bash"`, `"posix"`, `"mksh"`, `"zsh"`.                         |
| `indentWidth`      | number  | `8`      | Number of spaces for indent. Inherits dprint global config if set, or falls back to `8`. |
| `useTabs`          | boolean | `true`   | Indent with tabs. Inherits dprint global config if set, or falls back to `true`.         |
| `binaryNextLine`   | boolean | `false`  | Put binary operators (`&&`, `\|\|`, `\|`) at start of next line instead of end.          |
| `switchCaseIndent` | boolean | `false`  | Indent `case` pattern arms under the `case` statement.                                   |
| `spaceRedirects`   | boolean | `false`  | Put a space between redirect operators and target (e.g. `> file` instead of `>file`).    |
| `keepPadding`      | boolean | `false`  | Keep column alignment spaces between tokens.                                             |
| `functionNextLine` | boolean | `false`  | Put function opening brace `{` on a new line (Allman style).                             |
| `neverSplit`       | boolean | `false`  | Keep statements on a single line where possible.                                         |
| `simplify`         | boolean | `false`  | Rewrite redundant syntax to simpler forms (e.g. `${foo}` to `$foo`).                     |
| `minify`           | boolean | `false`  | Remove comments and extra whitespace to make code small.                                 |

### Indentation and Global Config

In dprint, top-level settings in `dprint.json` (such as `"indentWidth": 2` and `"useTabs": false`) apply to all plugins by default.

If your project sets global options for other languages (like TypeScript or JSON), this plugin inherits them. If you want shell scripts to keep the standard shfmt style (tabs), override them in the `"shuck"` block:

```json
{
  "indentWidth": 2,
  "useTabs": false,
  "shuck": {
    "useTabs": true
  }
}
```

### Dialect Resolution

When `dialect` is `"auto"` (default), this plugin chooses the dialect in this order:

1. **Shebang**: Reads the first line `#!...` (e.g., `#!/usr/bin/env bash` -> Bash, `#!/usr/bin/env zsh` -> Zsh).
2. **File extension**:
   - `.bash`, `.bats` -> Bash
   - `.zsh` -> Zsh
   - `.sh`, `.dash`, `.ksh` -> Posix
   - `.mksh` -> Mksh
3. **Fallback**: Files without an extension (like `.envrc`) default to Bash.

> [!NOTE]
> `.sh` files without a shebang resolve to `posix` by default.
> If your `.sh` file uses Bash syntax (such as `[[ ]]`), parsing will fail in `posix` mode.
> To fix this, set `"dialect": "bash"` in your configuration or add a `#!/usr/bin/env bash` shebang line.

### Supported Files

- File extensions: `.sh`, `.bash`, `.zsh`, `.ksh`, `.mksh`, `.dash`, `.bats`
- File names: `.envrc`, `.bashrc`, `.bash_profile`, `.bash_login`, `.bash_aliases`, `.bash_logout`, `.profile`, `.zshrc`, `.zshenv`, `.zprofile`, `.zlogin`, `.zlogout`

Unix tools often use shell scripts in extensionless files like dotfiles.
This repository covers some popular file names above, but does not plan to keep adding more default file names.
When formatting other extensionless scripts for your use case, use dprint's [`shebangs`](https://dprint.dev/config/#shebangs) configuration first:

```json
{
  "shebangs": {
    "#!/bin/sh": "sh",
    "#!/usr/bin/env bash": "bash",
    "#!/usr/bin/env zsh": "zsh"
  }
}
```

> [!WARNING]
> Upstream `shuck-formatter` only infers dialects from shebangs and file extensions, falling back to Bash for extensionless files.
> This plugin bridges this gap by detecting known Zsh dotfiles (like `.zshrc`) and passing `zsh` dialect upstream.
> This workaround is heuristic. If you encounter unexpected formatting behavior, set `"dialect": "zsh"` explicitly.

Non-shell files such as `Makefile` are not supported.

## Tips

To format code blocks in markdown, use the `tags` option in the official markdown plugin:

```bash
dprint add markdown
```

In `tags`:

- **Key**: The code block language tag in Markdown (e.g. `bash`).
- **Value**: The target file extension (without a period) that tells this plugin how to format the code.

For example, `.envrc` is a file name, not an extension. So map `"envrc"` to `"bash"`.

```json
{
  "markdown": {
    "tags": {
      "sh": "sh",
      "bash": "bash",
      "zsh": "zsh",
      "envrc": "bash",
      "shell": "sh",
      "shell-script": "sh"
    }
  }
}
```

This list satisfies linguist [mapping](https://github.com/github-linguist/linguist/blob/90fe0515940a5192980b2545c33ec59ef9fef17b/lib/linguist/languages.yml#L7468-L7476).

## JS Formatting API

- [JS Formatter](https://github.com/dprint/js-formatter) - Browser/Deno and Node
- [npm package](https://www.npmjs.com/package/@dprint/shuck)

## Versioning

This repo automatically upgrades to the latest version of shuck-formatter once a day. You can check which version of shuck-formatter is being used by looking at the `shuck-formatter` entry in the Cargo.toml file in this repo:

https://github.com/dprint/dprint-plugin-shuck/blob/main/Cargo.toml

At the moment, the version of this plugin does not reflect the version of shuck-formatter.

## Acknowledgments

- Thanks to [shuck](https://github.com/ewhauser/shuck) for making it easy to use as a dprint Wasm plugin.
- Thanks to [shfmt](https://github.com/mvdan/sh) for the reference and the original formatter.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
