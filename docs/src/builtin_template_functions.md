# Template functions

Yolk template tags simply execute Lua functions that transform the block of text the tag operates on.

Quick reminder: Yolk has three different types of tags, that differ only in what text they operate on:
- Next-line tags (`{# ... #}`): These tags operate on the line following the tag.
- Inline tags (`{< ... >}`): These tags operate on everything before the tag within the same line.
- Block tags (`{% ... %} ... {% end %}`): These tags operate on everything between the tag and the corresponding `{% end %}` tag.

Inside these tags, you can call any of Yolks template tag functions (Or, in fact, any Lua expression that returns a string).

## Built-in Template Functions

### `replace(pattern, replacement)`

> Replaces all occurrences of a Regex `pattern` with `replacement` in the text.
>
> (shorthand: `r(pattern, replacement)`).
>
> #### Example
>
> ```handlebars
> ui_font = "Arial" # {< replace(`".*"`, `"{data.font.ui}"`) >}
> ```
>
> Note that the replacement value needs to contain the quotes, as those are also matched agains in the regex pattern.
> Otherwise, we would end up with invalid toml.

### `replace_in(delimiter, replacement)`

> (shorthand: `ri(delimiter, replacement)`).
>
> Replaces the text between two delimiters with the `replacement`.
>
> #### Example
>
> ```handlebars
> ui_font = "Arial" # {< replace_in(`"`, data.font.ui) >}
> ```
>
> Note: we don't need to include the quotes in the replacement here.


### `replace_color(new_hex_color)`

> (shorthand: `rc(new_hex_color)`).
>
> Replaces a hex color value with a new hex color.
>
> #### Example
>
> ```handlebars
> background_color = "#282828" # {< replace_color(data.colors.bg) >}
> ```
