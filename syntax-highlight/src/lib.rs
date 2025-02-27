use harfbuzz_wasm::{debug, Font, GlyphBuffer};
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::easy::HighlightLines;
use wasm_bindgen::prelude::*;

// Pre-defined color palette indices
// These match the CPAL (Color Palette) table in the font
const PALETTE_DEFAULT: u8 = 0;  // #F8F8F2 - Default text
const PALETTE_KEYWORD: u8 = 1;  // #F92672 - Keywords (if, function, etc.)
const PALETTE_NUMBER: u8 = 2;   // #AE81FF - Numeric literals
const PALETTE_COMMENT: u8 = 3;  // #75715E - Comments
const PALETTE_FUNCTION: u8 = 4; // #66D9EF - Function names
const PALETTE_STRING: u8 = 2;   // Reuse #AE81FF for strings since we're missing a color

fn get_color_index(token_type: &str) -> u8 {
    if token_type.contains("keyword") {
        PALETTE_KEYWORD
    } else if token_type.contains("string") {
        PALETTE_STRING  // We're using the same color as numbers
    } else if token_type.contains("numeric") || token_type.contains("constant.numeric") {
        PALETTE_NUMBER
    } else if token_type.contains("comment") {
        PALETTE_COMMENT
    } else if token_type.contains("function") || token_type.contains("entity.name.function") {
        PALETTE_FUNCTION
    } else {
        PALETTE_DEFAULT
    }
}

#[wasm_bindgen]
pub fn shape(
    _shape_plan: u32,
    font_ref: u32,
    buf_ref: u32,
    _features: u32,
    _num_features: u32,
) -> i32 {
    // Send an initial debug message to show the WASM module is loaded and running
    debug("WASM module started...");
    
    let font = Font::from_ref(font_ref);
    let mut buffer = GlyphBuffer::from_ref(buf_ref);
    
    // Convert buffer to string for syntax highlighting
    let buf_u8: Vec<u8> = buffer.glyphs.iter().map(|g| g.codepoint as u8).collect();
    let input_text = String::from_utf8_lossy(&buf_u8);
    
    debug(&format!("Input text: {}", input_text));
    debug(&format!("Buffer has {} glyphs", buffer.glyphs.len()));
    
    // Load syntax definitions and theme
    debug("Loading syntax definitions...");
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    
    // Get JavaScript syntax definition
    debug("Finding JavaScript syntax...");
    let syntax = ps.find_syntax_by_extension("js").unwrap_or_else(|| {
        // Fallback to first syntax if JavaScript not found
        debug("JavaScript syntax not found, using default");
        &ps.syntaxes()[0]
    });
    debug(&format!("Using syntax: {}", syntax.name));
    
    // Create highlighter with Monokai theme
    debug("Setting up highlighter with Monokai theme...");
    let theme = &ts.themes["Monokai"];
    let mut h = HighlightLines::new(syntax, theme);
    
    // Get highlighted regions
    debug("Highlighting text...");
    let highlighted = match h.highlight_line(&input_text, &ps) {
        Ok(h) => h,
        Err(e) => {
            debug(&format!("Error highlighting: {:?}", e));
            return 0;
        }
    };
    
    debug(&format!("Found {} highlighted regions", highlighted.len()));
    
    // Map characters to glyphs and store color info in the flags field
    debug("Mapping characters to glyphs and setting colors...");
    let mut debug_color_counts = [0; 5]; // Track how many glyphs get each color
    
    for (i, item) in buffer.glyphs.iter_mut().enumerate() {
        // Find the style for this character position
        let mut color_index = PALETTE_DEFAULT;
        
        let mut pos = 0;
        for (style, text) in &highlighted {
            let text_len = text.chars().count();
            if i >= pos && i < pos + text_len {
                // This character is within this style range
                // In newer syntect, we need to check token type differently
                // The foreground color identifies the token type
                let fg_color = style.foreground;
                debug(&format!("Character at pos {}: color is RGB({}, {}, {}, {})", 
                    i, fg_color.r, fg_color.g, fg_color.b, fg_color.a));
                
                // Match on RGB values from Monokai colors
                let token_type = match (fg_color.r, fg_color.g, fg_color.b) {
                    (249, 38, 114) => "keyword", // #F92672 - Keywords
                    (174, 129, 255) => "numeric", // #AE81FF - Numbers/Strings
                    (117, 113, 94) => "comment", // #75715E - Comments
                    (102, 217, 239) => "function", // #66D9EF - Functions
                    _ => "",
                };
                
                debug(&format!("Token type: {}", token_type));
                color_index = get_color_index(token_type);
                break;
            }
            pos += text_len;
        }
        
        debug_color_counts[color_index as usize] += 1;
        
        // Map character to glyph
        let old_codepoint = item.codepoint;
        item.codepoint = font.get_glyph(item.codepoint, 0);
        
        // Store color palette index in the flags field
        // This will be used by HarfBuzz to select colors from CPAL table
        item.flags = color_index as u32;
        
        // For debugging purposes, force a more visible color mapping
        // This will make all keywords bright pink (palette index 1)
        if i % 5 == 0 {
            item.flags = 1; // Force every 5th glyph to be keyword color (pink)
        } else if i % 5 == 1 {
            item.flags = 2; // Force every 5th+1 glyph to be number color (purple)
        } else if i % 5 == 2 {
            item.flags = 3; // Force every 5th+2 glyph to be comment color (gray)
        } else if i % 5 == 3 {
            item.flags = 4; // Force every 5th+3 glyph to be function color (blue)
        }
        
        // Set advance width
        item.x_advance = font.get_glyph_h_advance(item.codepoint);
        
        // Log every 10th glyph for debug purposes
        if i % 10 == 0 {
            debug(&format!("Glyph {}: codepoint {} -> {}, color index: {}, x_advance: {}", 
                i, old_codepoint, item.codepoint, color_index, item.x_advance));
        }
    }
    
    debug(&format!("Color distribution - Default: {}, Keyword: {}, Number/String: {}, Comment: {}, Function: {}", 
        debug_color_counts[0], debug_color_counts[1], debug_color_counts[2], 
        debug_color_counts[3], debug_color_counts[4]));
    
    debug("WASM processing complete");
    
    // Return success
    1
}