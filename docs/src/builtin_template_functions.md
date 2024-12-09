# Template functions

Yolk template tags simply execute Lua functions that transform the block of text the tag operates on.

Quick reminder: Yolk has three different types of tags, that differ only in what text they operate on:
- Next-line tags (`{# ... #}`): These tags operate on the line following the tag.
- Inline tags (`{< ... >}`): These tags operate on everything before the tag within the same line.
- Block tags (`{% ... %} ... {% end %}`): These tags operate on everything between the tag and the corresponding `{% end %}` tag.

Inside these tags, you can call any of Yolks template tag functions (Or, in fact, any Lua expression that returns a string).

## Built-in Template Functions

### `replace_re(pattern, replacement)`

> Replaces all occurrences of a Regex `pattern` with `replacement` in the text.
>
> (shorthand: `rr(pattern, replacement)`).
>
> #### Example
>
> ```handlebars
> ui_font = "Arial" # {< replace_re(`".*"`, `"{data.font.ui}"`) >}
> ```
>
> Note that the replacement value needs to contain the quotes, as those are also matched agains in the regex pattern.
> Otherwise, we would end up with invalid toml.

### `replace_in(delimiter, replacement)`

> (shorthand: `rin(delimiter, replacement)`).
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


### `replace_between(left, right, replacement)`

> (shorthand: `rbet(left, right, replacement)`).
>
> Replaces the text between two delimiters with the `replacement`.
>
> #### Example
>
> ```handlebars
> ui_font = (Arial) # {< replace_between(`(`, `)`, data.font.ui) >}
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


### `replace_number(new_number)`

> (shorthand: `rnum(new_number)`).
>
> Replaces a number with another number.
>
> #### Example
>
> ```handlebars
> cursor_size = 32 # {< replace_number(data.cursor_size) >}
> ```


### `replace_quoted(value)`

> (shorthand: `rq(value)`).
>
> Replaces a value between quotes with another value
>
> #### Example
>
> ```handlebars
> ui_font = "Arial" # {< replace_quoted(data.font.ui) >}
> ```


### `replace_value(value)`

> (shorthand: `rv(value)`).
>
> Replaces a value (without spaces) after a `:` or a `=` with another value
>
> #### Example
>
> ```handlebars
> ui_font = Arial # {< replace_value(data.font.ui) >}
> ```
