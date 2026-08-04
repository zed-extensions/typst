# Typst Extension for Zed

## Usage

To register the LSP and enable certain features (such as compile on save), add the following to your local (resp. server) configuration file (`~/.zed/settings.json`):

```jsonc
// In settings.json
{
	"lsp": {
		"tinymist": {
			"initialization_options": {
				// Enable background preview, served on 127.0.0.1:23635.
				// See the Preview section below before relying on this.
				"preview": {
					"background": {
						"enabled": true,
					},
				},
			},

			"settings": {
				// Compile on save
				// This will compile a PDF for the `main.typ` file in the project root.
				"exportPdf": "onSave",
				"outputPath": "$root/$name",

				// Enable formatter
				"formatterMode": "typstyle",
			}
		}
	},

	"languages": {
		"Typst": {
			// Disable soft wrap
			"soft_wrap": "none",
		},
	},
}
```

To see all available options refer to [the tinymist documentation](https://github.com/Myriad-Dreamin/tinymist/blob/main/editors/neovim/Configuration.md).
Beware that the configuration options displayed there apply to **NeoVim**, not Zed, so some might be incorrect or misleading

## Preview

With `preview.background` enabled as shown above, tinymist serves a live preview from inside the
language server on <http://127.0.0.1:23635/>. Open it with the **`Typst: Open Preview`** task
(`task: spawn` in the command palette). It re-renders as you type, without saving.

### The preview follows one document at a time

The background preview tracks whichever document the language server most recently saw opened.
**Opening a second `.typ` file re-points the preview to that file**, and closing that file again does
not hand the preview back — edits to your original document then produce no preview updates at all.

To bring the preview back to a document, close its tab and reopen it. That re-sends
`textDocument/didOpen`, which is what tinymist keys on, and the update is incremental rather than a
full re-render.

tinymist pins the previewed document with its `tinymist.pinMain` command, but Zed has no built-in
button for it and cannot be told to send an arbitrary `workspace/executeCommand`. Two things were
tried and ruled out as substitutes: the background preview refuses `--not-primary`
(`cannot start background preview as non-primary`) and exposes no control-plane port, so it cannot be
re-pointed out of band; and setting an entry via `typstExtraArgs`, naming an input file in
`preview.background.args`, and `projectResolution: "lockDatabase"` were all tried and none of them
pin it either.

**[tinymist#2685](https://github.com/Myriad-Dreamin/tinymist/pull/2685)** (pending as of this
writing) adds a **"📌 Pin Preview to This File"** code lens whose command is `tinymist.pinMain`
directly, so any standards-compliant LSP client can invoke it — no special Zed support needed. Once
it ships, enable code lenses to use it:

```jsonc
// In settings.json
{
	"code_lens": "menu", // or "on"; Zed defaults to "off"
}
```

then run it from the code action menu (or inline, with `"on"`) at the top of the file. Until then,
the workaround above (close and reopen the tab) is the only recovery.

Note also that the preview shares the language server's lifetime. Zed restarts tinymist on every
`settings.json` save and every window reload, which drops the preview WebSocket without a close
frame; the page reconnects and re-downloads the whole document, which on a large document means
megabytes and a full re-render.

### Alternative: a preview pinned to one file

If you need several `.typ` files open at once, or a preview that survives language-server restarts,
run the **`Typst: Preview This File`** task instead. It starts an independent `tinymist preview`
process bound to `$ZED_FILE`, so nothing can steal it. Disable `preview.background` first, or the two
will fight over port 23635.

That process renders the file *on disk* rather than your buffer, so pair it with autosave:

```jsonc
// In settings.json
{
	"autosave": { "after_delay": { "milliseconds": 500 } },
}
```

## Components

- Tree Sitter: [tree-sitter-typst](https://github.com/uben0/tree-sitter-typst/)
- Language Server: [tinymist](https://github.com/Myriad-Dreamin/tinymist/)
