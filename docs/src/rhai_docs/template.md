<div class='rhai-doc'>

# Template tag functions

Yolk template tags simply execute rhai functions that transform the block of text the tag operates on.

Quick reminder: Yolk has three different types of tags, that differ only in what text they operate on:

- Next-line tags (`{# ... #}`): These tags operate on the line following the tag.
- Inline tags (`{< ... >}`): These tags operate on everything before the tag within the same line.
- Block tags (`{% ... %} ... {% end %}`): These tags operate on everything between the tag and the corresponding `{% end %}` tag.

Inside these tags, you can call any of Yolks template tag functions (Or, in fact, any rhai expression that returns a string).

---

**namespace**: `template`

---



<div class='doc-block'>

## replace_between

<div class='doc-content'>

```rust,ignore
replace_between(left: &str, right: &str, replacement: &str) -> Result<String>
```

**shorthand**: `rbet`.

Replaces the text between two delimiters with the `replacement`.

#### Example

```handlebars
ui_font = (Arial) # {< replace_between(`(`, `)`, data.font.ui) >}
```

Note: we don't need to include the quotes in the replacement here.

</div>
</div>




<div class='doc-block'>

## replace_color

<div class='doc-content'>

```rust,ignore
replace_color(replacement: &str) -> Result<String>
```

**shorthand**: `rcol`.

Replaces a hex color value with a new hex color.

#### Example

```handlebars
background_color = "#282828" # {< replace_color(data.colors.bg) >}
```

</div>
</div>




<div class='doc-block'>

## replace_in

<div class='doc-content'>

```rust,ignore
replace_in(between: &str, replacement: &str) -> Result<String>
```

**shorthand**: `rin`.

Replaces the text between two delimiters with the `replacement`.

#### Example

```toml
ui_font = "Arial" # {< replace_in(`"`, data.font.ui) >}
```

Note: we don't need to include the quotes in the replacement here.

</div>
</div>




<div class='doc-block'>

## replace_number

<div class='doc-content'>

```rust,ignore
replace_number(replacement: Dynamic) -> Result<String>
```

**shorthand**: `rnum`.

Replaces a number with another number.

#### Example

```handlebars
cursor_size = 32 # {< replace_number(data.cursor_size) >}
```

</div>
</div>




<div class='doc-block'>

## replace_quoted

<div class='doc-content'>

```rust,ignore
replace_quoted(replacement: &str) -> Result<String>
```

**shorthand**: `rq`.

Replaces a value between quotes with another value

#### Example

```handlebars
ui_font = "Arial" # {< replace_quoted(data.font.ui) >}
```

</div>
</div>




<div class='doc-block'>

## replace_re

<div class='doc-content'>

```rust,ignore
replace_re(regex: &str, replacement: &str) -> Result<String>
```

**shorthand**: `rr`.

Replaces all occurrences of a Regex `pattern` with `replacement` in the text.

#### Example

```handlebars
ui_font = "Arial" # {< replace_re(`".*"`, `"{data.font.ui}"`) >}
```

Note that the replacement value needs to contain the quotes, as those are also matched against in the regex pattern.
Otherwise, we would end up with invalid toml.

</div>
</div>




<div class='doc-block'>

## replace_value

<div class='doc-content'>

```rust,ignore
replace_value(replacement: &str) -> Result<String>
```

**shorthand**: `rv`.

Replaces a value (without spaces) after a `:` or a `=` with another value

#### Example

```handlebars
ui_font = Arial # {< replace_value(data.font.ui) >}
```

</div>
</div>




</div>