# Conditionals

Yolk allows you to conditionally include parts of your configuration based on the state of your system.
It achieves this by commenting or un-commenting blocks of your file.

Conditionals use special template tags that start with the keyword `if`,
which instructs Yolk to treat the following expression as a conditional,
rather than a regular template tag function.

## Multiline conditionals

The most common form of conditional block is using the multiline template tag syntax.
Let's type out a simple example:

```toml
# {% if SYSTEM.hostname == "epic-desktop" %}
displays = ["DP-1", "DP-2"]
# {% if SYSTEM.hostname == "business-desktop" %}
displays = ["HDMI-1", "HDMI-2"]
# {% else %}
displays = ["eDP-1"]
# {% end %}
```

Now, this is of course not a valid configuration just yet, as we're setting the `displays` variable thrice.

However, once you run `yolk sync`, yolk will evaluate the condition and comment out the blocks that don't apply.
For example, on your laptop, this config might be turned into:

```toml
# {% if SYSTEM.hostname == "epic-desktop" %}
#<yolk> displays = ["DP-1", "DP-2"]
# {% if SYSTEM.hostname == "business-desktop" %}
#<yolk> displays = ["HDMI-1", "HDMI-2"]
# {% else %}
displays = ["eDP-1"]
# {% end %}
```

Note that yolk added a special `<yolk>` prefix to the comments.
Yolk conditionals will only ever add or remove comments with this prefix,
which means that you can still have regular comments in those conditional blocks.

## Inline and Next-line conditionals

A more simplified version of this is also supported in inline and next-line tags:

```kdl
enable_variable_refreshrate // {< if data.is_desktop() >}

// {# if data.enable_xwayland #}
spawn-at-startup "xwayland-satellite"
```
