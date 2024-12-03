# Luau

Yolk uses a special dialect of Lua called [Luau](https://luau.org).
The differences from regular Lua are very minimal, so you can mostly ignore the fact that we're using Luau.

However, Luau brings a couple of features that are very convenient when using Yolk.
You can find a list of all main syntax differences here: [Luau Syntax](https://luau-lang.org/docs/syntax).

The most useful features are listed below.

## Template literals
Luau supports template literals, which allow you to to interpolate variables into strings comfortably.
Simply surround your strings with backticks and embed any lua expression inside your string with `{}`, like so:

```lua
local message = `Hello {person.firstname} {person.lastname}`
```

## if-else expression
Luau supports using `if`/`else` in expression position, like a typical ternary operator.

```lua
local background_color = if theme == "dark" then
  "#000000"
else
  "#FFFFFF"
```
