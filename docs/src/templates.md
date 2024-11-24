# Templates in Yolk

Yolk allows you to use simple templates directly within your config files.
Those templates will be evaluated whenever you run `yolk sync`, use a new egg, or interact with git through yolk.

Expressions within these templates are written in the [Rhai](https://rhai.rs) scripting language,
and have access to a couple special variables that allow you to reference your configuration and system state.

## Conditional
Let's take a look at a simple example of conditional template syntax:
```toml
# {% if system.hostname == "epic-desktop" %}
displays = ["DP-1", "DP-2"]
# {% else %}
displays = ["eDP-1"]
# {% end %}
```
Once you run `yolk sync`, yolk will evaluate the condition and comment out the block that doesn't apply.
For example, on your laptop, this config would be turned into:
```toml
# {% if system.hostname == "epic-desktop" %}
#<yolk> displays = ["DP-1", "DP-2"]
# {% else %}
displays = ["eDP-1"]
# {% end %}
```

## Value replacement
In many cases, you'll want to make specific values, like colors or paths, be set through one central source, rather than specifying them in every config file.
Yolk allows you to do this (and more) by using the `replace` directive.
This directive takes a regex pattern and a Rhai expression, and replaces the matched pattern with the result of the expression.
```toml
# {% replace /".*"/ `"${colors.background}"`%}
background_color = "#000000"
# {% replace /".*"/ `"${colors.foreground}"`%}
foreground_color = "#ffffff"
```
After running `yolk sync`, yolk will replace the regex patterns with the corresponding result of the Rhai expression.
For example, depending on how you configured your `colors` in your `yolk.rhai`, this could turn into:
```toml
# {% replace /".*"/ `"${colors.background}"`%}
background_color = "#282828"
# {% replace /".*"/ `"${colors.foreground}"`%}
foreground_color = "#ebdbb2"
```
Note that the expression here still needs to contain the quotes, to continue returning valid toml.
Yolk will refuse to evaluate `replace` directives that are non-reversible (i.e. if you replaced `".*"` with `foo`, as `foo` will no longer match that regex pattern).