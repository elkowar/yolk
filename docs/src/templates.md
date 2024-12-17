# Templates in Yolk

Yolk allows you to use simple templates directly within your config files.
Those templates will be evaluated whenever you run `yolk sync` or interact with git (see [Git concepts](./git_concepts.md)).

Expressions within these templates are written in the [Rhai](https://rhai.rs) scripting language,
and have access to a couple special variables that allow you to reference your configuration and system state.

There are two main kinds of templates in Yolk: [conditional templates](./conditional_templates.md) and [template tag functions](#template-tag-functions).

## Preparation
To make yolk evaluate your file as a template, you need to explicitly tell yolk about it.
To do this, make sure you include it in the `templates` list of your eggs deployment configuration in your `yolk.rhai`:

```rust,ignore
export let eggs = #{
  foo: {
    targets: "~/.config/foo",
    templates: ["foo.toml"],
  }
}
```


## Conditionals

You can use [Conditional template elements](./conditional_templates.md) to conditionally include or exclude parts of your configuration.
Let's take a look at a simple example:

```toml
# {% if SYSTEM.hostname == "epic-desktop" %}
displays = ["DP-1", "DP-2"]
# {% else %}
displays = ["eDP-1"]
# {% end %}
```

Once you run `yolk sync`, yolk will evaluate the condition and comment out the block that doesn't apply.
For example, on your laptop, this config would be turned into:
```toml
# {% if SYSTEM.hostname == "epic-desktop" %}
#<yolk> displays = ["DP-1", "DP-2"]
# {% else %}
displays = ["eDP-1"]
# {% end %}
```

## Template tag functions

In addition to conditionals, Yolk provides a wide variety of functions to _edit_ your configuration, such as to insert values from, say, your color theme.
These are regular rhai functions that modify their attached block of text (see below).

All of the built-in template tag functions are documented in the [Rhai reference](./rhai_docs/template.md),
and you can also [define your own!](./custom_template_functions.md)

### Different types of tags

Yolk supports three different types of tags:
- Next-line tags (`{# ... #}`): These tags operate on the line following the tag.
- Inline tags (`{< ... >}`): These tags operate on everything before the tag within the same line.
- Block tags (`{% ... %} ... {% end %}`): These tags operate on everything between the tag and the corresponding `{% end %}` tag.

You can use whichever of these you want, wherever you want. For example, all of these do the same:
```toml
background_color = "#000000" # {< replace_color(colors.background) >}

# {# replace_color(colors.background) #}
background_color = "#000000"

# {% replace_color(colors.background) %}
background_color = "#000000"
# {% end %}
```

### Example: Templating your color scheme
In many cases, you'll want to make specific values, such as colors or paths, be set through one central source, rather than specifying them in every config file.
Yolk allows you to do this (and more) by using various template functions.
For example, the `replace_quoted` directive takes any value and replaces whatever is in quotes with the result of the expression.
```toml
# {# replace_quoted(colors.background) #}
background_color = "#000000"
# {# replace_quoted(colors.foreground) #}
foreground_color = "#ffffff"
```
After running `yolk sync`, yolk will replace the regex patterns with the corresponding result of the Lua expression.
For example, depending on how you configured your `colors` in your `yolk.rhai`, this could turn into:
```toml
# {# replace_quoted(colors.background) #}
background_color = "#282828"
# {# replace_quoted(colors.foreground) #}
foreground_color = "#ebdbb2"
```
Yolk will refuse to evaluate directives that are non-reversible (i.e. if you `replace_re`d `".*"` with `foo`, as `foo` will no longer match that regex pattern).
