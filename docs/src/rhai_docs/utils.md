# Utility functions

A collection of utility functions

---

**namespace**: `utils`

---

## color_hex_to_rgb

```rust,ignore
color_hex_to_rgb(hex_string: &str) -> Result<Map>
```

> Convert a hex color string to an RGB map.

---
## color_hex_to_rgb_str

```rust,ignore
color_hex_to_rgb_str(hex_string: &str) -> Result<String>
```

> Convert a hex color string to an RGB string.

---
## color_hex_to_rgba_str

```rust,ignore
color_hex_to_rgba_str(hex_string: &str) -> Result<String>
```

> Convert a hex color string to an RGBA string.

---
## color_rgb_to_hex

```rust,ignore
color_rgb_to_hex(rgb_table: Map) -> Result<String>
```

> Convert an RGB map to a hex color string.

---
## regex_captures

```rust,ignore
regex_captures(pattern: &str, s: &str) -> Result<Option<Vec<String>>>
```

> Match a string against a regex pattern and return the capture groups as a list.

---
## regex_match

```rust,ignore
regex_match(pattern: &str, haystack: &str) -> Result<bool>
```

> Check if a given string matches a given regex pattern.

---
## regex_replace

```rust,ignore
regex_replace(pattern: &str, haystack: &str, replacement: &str) -> Result<String>
```

> Replace a regex pattern in a string with a replacement.

---
