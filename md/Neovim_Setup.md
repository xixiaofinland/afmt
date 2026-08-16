# Config afmt in Neovim

There's no dedicated afmt plugin for Neovim, but afmt's stdin mode (`afmt -`)
was built for exactly this: it reads a buffer, writes only the formatted
source to stdout, and errors go to stderr. That's the contract a Neovim
formatter-runner plugin expects, so this guide wires afmt into one —
[conform.nvim](https://github.com/stevearc/conform.nvim), the most widely
used one. (The same `afmt` command/args also work as a `none-ls.nvim`
source, if you use that instead.)

This guide assumes no prior Neovim configuration experience — it spells out
where files live and what each snippet does, not just the snippets.

## Before you start

- **Check your Neovim version:** run `nvim --version` in a terminal. The
  first line looks like `NVIM v0.12.4`. This guide's simplest path (step 2,
  Option A) needs 0.12 or newer. If you're on an older version, see Option B
  in step 2 instead.
- **Make sure `afmt` itself works first**, outside of Neovim entirely: run
  `afmt --version` in a terminal. If that fails, fix it before touching
  Neovim — see the main [installation instructions](../README.md#-installation).
  Any failure after this point is a Neovim/conform configuration problem,
  not an afmt problem.

## 1. Find (or create) your Neovim config file

Neovim's configuration is a single Lua script, `init.lua`, that runs top to
bottom every time Neovim starts. It normally lives at:

- Linux/macOS: `~/.config/nvim/init.lua`
- Windows: `~/AppData/Local/nvim/init.lua`

To find out where *your* Neovim is actually looking (in case of an unusual
setup), start Neovim and run:

```
:echo $MYVIMRC
```

If that file doesn't exist yet, you're starting from nothing — create it:

```bash
mkdir -p ~/.config/nvim
touch ~/.config/nvim/init.lua
```

You can edit `init.lua` with any text editor, not necessarily Neovim itself.
Every snippet below gets added to this one file, in any order, unless
otherwise noted.

## 2. Get a plugin manager, so Neovim can download conform.nvim

conform.nvim isn't part of Neovim — it's a separate project
(github.com/stevearc/conform.nvim) that has to be downloaded and loaded on
startup. Doing that is what a "plugin manager" is for. Pick one:

### Option A — Neovim's built-in `vim.pack` (Neovim 0.12+, nothing extra to install)

As of Neovim 0.12, downloading and loading plugins is built into the editor
itself, via `vim.pack`. Add this near the top of `init.lua`:

```lua
vim.pack.add {
  'https://github.com/stevearc/conform.nvim',
}
```

Save the file and (re)start Neovim. On that first launch, Neovim clones the
repo to `~/.local/share/nvim/site/pack/core/opt/conform.nvim` — you'll see
download progress messages. After that, `require('conform')` (used in the
next steps) works from anywhere in your config.

### Option B — lazy.nvim (works on older Neovim; what most existing configs use)

If you already have a config built around
[lazy.nvim](https://github.com/folke/lazy.nvim) — look for a
`require('lazy').setup(...)` call in your `init.lua` — add conform.nvim as a
spec instead of using `vim.pack`:

```lua
require('lazy').setup {
  { 'stevearc/conform.nvim' },
  -- ...your other plugin specs
}
```

If you don't have lazy.nvim at all and are on an older Neovim, follow
lazy.nvim's own [installation instructions](https://github.com/folke/lazy.nvim#-installation)
first, then come back and add the spec above.

> The rest of this guide writes plain `require('conform')...` calls, which
> work the same regardless of which option you picked — Option A just needs
> them anywhere in `init.lua`, Option B typically wants them inside the
> plugin spec's `config = function() ... end`.

## 3. Tell Neovim what a `.cls` file is

This step is easy to skip and then wonder why nothing works: Neovim already
has a built-in rule for the `.cls` extension, and it isn't Apex. `.cls` is
also a LaTeX class-file extension and a Visual Basic class-module extension,
so Neovim inspects the file's *contents* to guess which one it is — and
falls back to `tex` when nothing matches, which is what happens to Apex
code. A plain "extension → filetype" mapping loses to that built-in guesser,
so this needs a `pattern` entry instead, which takes priority:

```lua
vim.filetype.add {
  extension = {
    trigger = 'apex',
    apexc = 'apex',
    apex = 'apex',
  },
  pattern = {
    ['.*%.cls'] = 'apex',
  },
}
```

`.trigger`, `.apex`, and `.apexc` aren't claimed by anything else in
Neovim, so a plain `extension` entry (the first part above) is enough for
those. Put this block anywhere in `init.lua`.

If you also want the official
[Apex Language Server](https://github.com/forcedotcom/salesforcedx-vscode)
for diagnostics/completion via `nvim-lspconfig`'s `apex_ls`, this same block
covers both — `apex_ls` and afmt's conform formatter share the `apex`
filetype. Note the language server doesn't do formatting itself
(`textDocument/formatting`), so you still need afmt (or Prettier Apex)
either way.

## 4. Configure conform.nvim to use afmt

afmt isn't one of conform.nvim's built-in formatters yet, so it's defined by
hand. Add this to `init.lua`:

```lua
require('conform').setup {
  formatters_by_ft = {
    apex = { 'afmt' },
  },
  formatters = {
    afmt = {
      command = 'afmt',
      args = function(self, ctx)
        local root = vim.fs.root(ctx.dirname, '.afmt.toml')
        if root then
          return { '--config', root .. '/.afmt.toml', '-' }
        end
        return { '-' }
      end,
    },
  },
}
```

What each part does:

- **`formatters_by_ft = { apex = { 'afmt' } }`** — tells conform.nvim "when
  the buffer's filetype is `apex`, run the formatter named `afmt`" (the
  filetype set up in step 3).
- **`formatters.afmt.command = 'afmt'`** — the executable to run. This must
  resolve on your `$PATH` exactly like the `afmt --version` check from
  "Before you start" — conform doesn't do anything special to locate it.
- **`formatters.afmt.args`** — the command-line arguments conform passes to
  `afmt`. It's a function (not a fixed list) because of one afmt quirk:
  stdin mode (`afmt -`) never auto-discovers a project's `.afmt.toml`, the
  way formatting an actual file on disk would — it has to be told about the
  config file explicitly with `--config`. So this function:
  - takes `ctx.dirname` (the directory of the file being formatted),
  - calls `vim.fs.root(...)` to walk *upward* from there looking for an
    `.afmt.toml`,
  - and if it finds one, passes `--config <path-to-it> -`; otherwise just
    passes `-` (format with afmt's defaults).

## 5. Run the formatter

**Manually, on demand** — from Neovim's command line (type `:`, then the
command, then Enter):

```
:lua require('conform').format({ async = false })
```

To avoid typing that every time, bind it to a key. `<leader>` is a
placeholder key most configs define (commonly Space or `\`; run
`:echo mapleader` to check yours):

```lua
vim.keymap.set({ 'n', 'v' }, '<leader>f', function()
  require('conform').format { async = true }
end, { desc = '[F]ormat buffer' })
```

> **This keymap only formats — it does not save.** It rewrites the buffer
> in memory but never calls `:w`. If you press it expecting the file on
> disk to change, nothing will, and if you then quit without saving
> (`:q!`), that in-memory reformat is discarded too. Use it to preview a
> reformat before committing to it; for your everyday workflow, save
> normally (`:w`) and let the "automatically, on save" setup below handle
> formatting as part of that write.

**Automatically, on save** — this autocmd formats any Apex file right
before it's written:

```lua
vim.api.nvim_create_autocmd('BufWritePre', {
  pattern = { '*.cls', '*.trigger', '*.apex', '*.apexc' },
  callback = function(args)
    require('conform').format { bufnr = args.buf }
  end,
})
```

(If your config already has a filetype-keyed `format_on_save` function —
this is how [kickstart.nvim](https://github.com/nvim-lua/kickstart.nvim)
does it, for example — you can add `apex = true` to that table instead of
adding a separate autocmd.)

## 6. Syntax highlighting (optional)

This part is unrelated to formatting and purely cosmetic — skip it if you
don't care about Apex syntax highlighting in Neovim.

[nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) ships
an `apex` parser built from the same
[tree-sitter-sfapex](https://github.com/aheber/tree-sitter-sfapex) grammar
afmt itself parses with. Once step 3 makes Neovim recognize the `apex`
filetype, most treesitter configs (kickstart.nvim included) auto-download
and compile that parser the first time you open a `.cls`/`.trigger` file —
no manual `:TSInstall apex` needed.

If that auto-install fails with an error mentioning
`Error during "tree-sitter build"` and `ENOENT ... 'tree-sitter'`, it means
the `tree-sitter` **CLI** isn't installed. This is a separate tool from the
`nvim-treesitter` plugin and from the `tree-sitter` C library Neovim itself
may depend on — newer nvim-treesitter shells out to it to compile parsers.
Install it with whatever package manager you have available, e.g.:

```bash
brew install tree-sitter-cli   # Homebrew
cargo install tree-sitter-cli  # Rust toolchain
```

then reopen the file.

## 7. Verify it works

1. Open an Apex file: `nvim path/to/something.cls`
2. Check the filetype: `:set filetype?` should print `filetype=apex`. If it
   prints something else (often `tex`), revisit step 3.
3. Format it: `:lua require('conform').format({ async = false })` (or your
   keymap from step 5) should reformat the buffer in place.

## Troubleshooting

- **`E5108` / errors mentioning `conform`** right after adding step 2's
  snippet — the plugin hasn't downloaded yet, or Neovim hasn't been
  restarted since adding it. Plugins load at startup; quit and reopen
  Neovim.
- **Conform reports no formatter configured / nothing happens on
  `<leader>f`** — the buffer's filetype probably isn't `apex`. Check with
  `:set filetype?` (step 3).
- **`.cls` files open as `tex` (or something else) despite step 3** — make
  sure the `vim.filetype.add` block actually ran (no typos, not
  accidentally wrapped in a conditional) and that nothing later in
  `init.lua` overrides the same pattern — the last matching registration
  wins.
- **Formatting runs but produces no change / an error about `afmt` not
  found** — confirm `afmt --version` works in a plain terminal first. If it
  doesn't, Neovim won't be able to run it either; conform.nvim doesn't
  install or locate `afmt` for you.
- **Treesitter install error mentioning `'tree-sitter'`** — see step 6.
- **Pressed `<leader>f`, saw no change on disk (or after quitting without
  saving)** — that keymap only formats the in-memory buffer, it doesn't
  write it. See the note under step 5. Save with `:w` instead — if
  format-on-save is configured, that both formats and writes in one
  synchronous step, so the change is visible immediately.
- **Formatting silently does nothing — no error, no change, no message**
  — the most common cause is that afmt can't parse the file (invalid
  Apex syntax: a missing `;`, an unbalanced brace, etc.) and exits with
  an error, which conform.nvim swallows by default. Check directly in a
  terminal:

  ```bash
  afmt path/to/file.cls
  ```

  If that prints a parse error (e.g. `Parser encounters an error node in
  the tree`), fix the syntax issue and try again — afmt intentionally
  refuses to format invalid code rather than guess. To see these errors
  inside Neovim instead of a terminal, set `notify_on_error = true` in
  your `conform.setup { ... }` call (it defaults to `false` in
  kickstart.nvim).
