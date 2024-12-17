<div class='rhai-doc'>

# Utility functions

A collection of utility functions

---

**namespace**: `utils`

---



<div class='doc-block'>

## color_hex_to_rgb

<div class='doc-content'>

```rust,ignore
color_hex_to_rgb(hex_string: &str) -> Result<Map>
```

Convert a hex color string to an RGB map.

</div>
</div>




<div class='doc-block'>

## color_hex_to_rgb_str

<div class='doc-content'>

```rust,ignore
color_hex_to_rgb_str(hex_string: &str) -> Result<String>
```

Convert a hex color string to an RGB string.

</div>
</div>




<div class='doc-block'>

## color_hex_to_rgba_str

<div class='doc-content'>

```rust,ignore
color_hex_to_rgba_str(hex_string: &str) -> Result<String>
```

Convert a hex color string to an RGBA string.

</div>
</div>




<div class='doc-block'>

## color_rgb_to_hex

<div class='doc-content'>

```rust,ignore
color_rgb_to_hex(rgb_table: Map) -> Result<String>
```

Convert an RGB map to a hex color string.

</div>
</div>




<div class='doc-block'>

## regex_captures

<div class='doc-content'>

```rust,ignore
regex_captures(pattern: &str, s: &str) -> Result<Option<Vec<String>>>
```

Match a string against a regex pattern and return the capture groups as a list.

</div>
</div>




<div class='doc-block'>

## regex_match

<div class='doc-content'>

```rust,ignore
regex_match(pattern: &str, haystack: &str) -> Result<bool>
```

Check if a given string matches a given regex pattern.

</div>
</div>




<div class='doc-block'>

## regex_replace

<div class='doc-content'>

```rust,ignore
regex_replace(pattern: &str, haystack: &str, replacement: &str) -> Result<String>
```

Replace a regex pattern in a string with a replacement.

</div>
</div>




</div>