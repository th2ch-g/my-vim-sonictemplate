# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Nature

Personal fork of `mattn/vim-sonictemplate`. The Vim engine code in `plugin/`, `autoload/`, `doc/`, `syntax/` is upstream and rarely changed. Almost all local commits add or tweak files under `template/` (and a small `ftdetect/gopass.vim` integration for editing gopass secrets in Vim).

When making changes, default assumption: **you are adding a template, not modifying the engine.** Touch `autoload/sonictemplate.vim` only if a behavior genuinely cannot be expressed in a template.

## How the Engine Works (read before editing templates)

The plugin exposes `:Template <name>` and `<c-y>t`. The dispatch in `autoload/sonictemplate.vim` does roughly this:

1. Determine **filetype** via `&filetype` plus a per-language `sonictemplate#lang#<ft>#guess()` hook (see `autoload/sonictemplate/lang/*.vim`) which can override filetype/prefix/filter from cursor context (e.g. inside `<script>` of HTML → `javascript`).
2. Determine **prefix** from buffer/mode:
   - `base-` — buffer is empty (whole-file scaffolds)
   - `snip-` — buffer has content (insert at cursor)
   - `wrap-` — visual mode, wraps the selection (exposed via `{{_wrap_}}`)
   - `file-` — matched only when current filename leads with the template name (e.g. `LICENSE` → `template/_/file-LICENSE-mit.txt`)
3. Glob `template/<ft>/<prefix>-<name>*.*` across each dir in `s:tmpldir` (user-configured dirs first, bundled `template/` last). Filetype `_` is the global fallback.
4. Read the chosen file, run substitutions, then either replace the buffer (`base`), insert relative to cursor with indentation matching, or expand inline (`{{_inline_}}`).

### Template Placeholders

Defined and processed in `sonictemplate#apply()` (`autoload/sonictemplate.vim:235`):

| Placeholder | Meaning |
|---|---|
| `{{_cursor_}}` | Final cursor position after expansion |
| `{{_name_}}` / `{{_name_:Default}}` | Buffer basename (sanitized to `[A-Za-z0-9_]`) |
| `{{_dir_}}` | Parent directory name (sanitized) |
| `{{_input_:var}}` / `{{_input_:var:default}}` | Prompt the user; cached in `b:` and global `g:sonictemplate_vim_vars` |
| `{{_var_:name}}` | Reuse a previously-input or `_define_` value |
| `{{_define_:name:vimexpr}}` | Compute a value via Vimscript expression |
| `{{_expr_:vimexpr}}` | Inline Vimscript expression (sandboxed) |
| `{{_if_:cond;then;else}}` | Ternary (sandboxed Vimscript condition) |
| `{{_wrap_}}` | Visual-selection contents (only meaningful for `wrap-` templates) |
| `{{_inline_}}` | Force one-line expansion at cursor (template newlines collapse) |
| `{{_filter_:tag}}` | Bias subsequent completion ordering toward templates whose name matches `tag` |
| `{{_lang_util_:...}}` | Calls `sonictemplate#lang#<ft>#util(...)` |

### Postfix Completion

Per-filetype `template/<ft>/pattern.stpl` files map regex → expansion (loaded by `sonictemplate#load_postfix()` at `autoload/sonictemplate.vim:471`). Tab-indented body lines define the replacement; `{{$1}}..{{$9}}` reference regex capture groups.

### `meta.json` (per-template-dir)

Optional `template/<ft>/meta.json` can declare `prefer-base`, `prefer-snip`, `prefer-file` arrays of regexes to bias the completion order (used by `s:sort` in `autoload/sonictemplate.vim:63`).

## Adding a Template — Checklist

1. Pick the filetype dir (`template/<ft>/`, or `_/` for global). Create it if new.
2. Name the file `<prefix>-<name>.<ext>` where `<prefix>` is `base`/`snip`/`file`/`wrap`. The first matching extension in the dir wins, so keep it consistent with existing siblings.
3. Use placeholders above; remember `{{_cursor_}}` to land the cursor and that `snip-*` templates get re-indented to the current line's indent.
4. If the local-only fork already has a `snip-` variant of similar shape (e.g. `python/snip-shebang*.py`), match its style rather than introducing a new convention.
5. Test by opening a scratch buffer of the right filetype and running `:Template <name>` (and `:Template <Tab>` to verify it appears in completion).

## Common Commands

```vim
" In a new buffer matching the filetype:
:Template <Tab>           " list candidates (base/file)
:Template <name>          " expand
" Default keymaps (see plugin/sonictemplate.vim):
<c-y>t                    " select via input prompt
<c-y>T                    " 'intelligent' variant — prefers syntax-derived ft over &filetype
<c-y><c-b>                " postfix expansion at cursor
```

There is no build, lint, or test toolchain — this is a pure Vimscript plugin. Verification = open Vim and run the template.

## Local Customizations of Note

- `ftdetect/gopass.vim` — sets `filetype=gopass` for gopass-edit temp files; paired templates in `template/gopass/`.
- `template/python/snip-shebang-unbf.py` — Python unbuffered shebang variant.
- `template/python/base-plot.py` — uv inline-script-metadata header (PEP 723) for matplotlib/pandas/seaborn one-shots. Per user preference, Python tooling here standardizes on `uv`.
- `template/markdown/snip-add-todo*.md` — `- [ ]` / `- [x]` line snippets.

When extending Python or shell templates, prefer `uv`-driven shebangs (`#!/usr/bin/env -S uv run --script` with PEP 723 metadata) over plain `python3` to match existing patterns.
