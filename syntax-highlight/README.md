# JavaScript Syntax Highlighting Font

This example demonstrates using the HarfBuzz WASM shaper to create a font that provides JavaScript syntax highlighting capabilities. The shaper uses a simplified version of [syntect](https://github.com/trishume/syntect), a syntax highlighting library for Rust.

## How it works

1. The font receives text input as a buffer of characters
2. The WASM shaper analyzes the text using a minimal JavaScript syntax definition
3. Tokens are categorized (keyword, string, number, comment, function, etc.)
4. The font flags field is used to store color palette index information
5. The rendered text automatically has syntax highlighting

## Font Requirements

This example uses the `flags` field of each glyph to store the color palette index. A color font using COLR/CPAL tables should be used so that HarfBuzz can apply the correct colors from the palette.

The color palette indices are:
- 0: Default text color (#F8F8F2)
- 1: Keywords (if, function, var, etc.) (#F92672)
- 2: Numbers and Strings (#AE81FF)
- 3: Comments (#75715E)
- 4: Functions (#66D9EF)

To create a complete working font, you would need to:

1. Start with a monospace font
2. Add COLR/CPAL tables with the appropriate color palettes
3. Package the font with the WASM module

## Example Usage

When using a text editor or viewer that supports HarfBuzz WASM shapers, JavaScript code will automatically be displayed with syntax highlighting:

```javascript
function calculateArea(radius) {
  // Calculate the area of a circle
  const PI = 3.14159;
  return PI * radius * radius;
}
```

## Notes on Implementation

The implementation includes:

1. A minimal JavaScript syntax definition to reduce the WASM size
2. Direct usage of the glyph's flags field for storing color information
3. Integration with HarfBuzz's color font support

This is a proof of concept and can be extended to support more languages or better syntax recognition in future versions.