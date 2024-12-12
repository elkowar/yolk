# Utility functions

Yolk provides a set of utility functions that can be used in your [yolk.lua](./yolk_lua.md) or your templates.

## Miscellaneous

### `bool regex_match(pattern, string)`

> Check if a given regex pattern is found in a string.

### `string regex_replace(pattern, string, replacement)`

> Replace all occurrences of a regex pattern in a string with a replacement.

### `string[] regex_captures(pattern, string)`

> Return capture group values from a regex match.

### `bool contains_value(table, value)`

> Check if a given table contains a given value

### `bool contains_key(table, value)`

> Check if a given table contains a given key

### `value from_json(json_string)`

> parse a json string

### `string to_json(value)`

> serialize a value into a json string

### `table color_hex_to_rgb(hex_str)`

> read a hex color string and return a table with r, g, b, a values

### `string color_hex_to_rgb_str(hex_str)`

> read a hex color string and return a CSS-style rgb string such as `rgb(255, 255, 255)

### `string color_hex_to_rgba_str(hex_str)`

> read a hex color string and return a CSS-style rgb string such as `rgba(255, 255, 255, 125)

### `string color_rgb_to_hex(rgb_table)`

> read a table with r, g, b, a fields and return a hex color string

## Environment and Filesystem

### `bool command_available(string)`

> Checks if a given executable is available on the system.

### `string env(name, default)`

> Read an environment variable, or the default value if it's not set.

### `bool path_exists(path)`

> Checks if a given path exists.

### `bool path_is_dir(path)`

> Checks if a given path exists and is a directory

### `bool path_is_file(path)`

> Checks if a given path exists and is a file

### `string read_file(path)`

> read the contents of the given file

### `string[] read_dir(path)`

> list children of a directory

## Inspect.lua

For convenience when debugging yolk includes the [inspect.lua](https://github.com/kikito/inspect.lua) library.

### Example

```lua
local inspect = require 'inspect'
print(inspect({a = 1, b = 2}))
```

Or:

```bash
$ yolk eval 'inspect.inspect(SYSTEM)'
```
