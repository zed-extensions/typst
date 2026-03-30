# Typst Extension for Zed

## Usage

To register the LSP and enable certain features (such as compile on save), add the following your `settings.json` (or `.zed/settings.json` in the project root for project-specific config),

```jsonc
// In settings.json
"lsp": {
  "tinymist": {
		"initialization_options": {
				// Enable background preview
				// Server will be running on 127.0.0.1:23635
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
      "outputPath": "$root/$name"
      
      // Enable formatter
      "formatterMode": "typstyle",
    }
  }
}
```


To see all available options refer to [the tinymist documentation](https://github.com/Myriad-Dreamin/tinymist/blob/main/editors/neovim/Configuration.md). 
Beware that the configuration options displayed there apply to **NeoVim**, not Zed, so some might be incorrect or misleading

## Components

- Tree Sitter: [tree-sitter-typst](https://github.com/uben0/tree-sitter-typst/)
- Language Server: [tinymist](https://github.com/Myriad-Dreamin/tinymist/)
