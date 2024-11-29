<div class="oranda-hide">

# Yolk – Painfree Templated Dotfile Management

</div>

Yolk is a dotfile management tool with a unique spin on templating,
sitting somewhere in between [GNU Stow](https://www.gnu.org/software/stow/) and [chezmoi](https://www.chezmoi.io/).

## WARNING

This is pre-alpha software. In its current state, do not trust it. At all.
Always make backups of everything before trying to use yolk, until it's in a stable state.

You have been warned.

## The Concept

Yolk allows you to use simple templates in your configuration files without having to worry about keeping a separate template file and the generated config file in sync.
This is achieved through a design that allows all templates to be included inside comments in your actual configuration file.

Let's demonstrate:
```toml
# Use a different font on one host
# {% if system.hostname == "epic-desktop" %}
font="Fira Code"
# {% else %}
#<yolk> font="Iosevka"
# {% end %}

[colors]
# Load your colors from your yolk configuration
# {% replace(`".*"`, `"${data.colors.background}"`) %}
background="#282828"
# {% replace(`".*"`, `"${data.colors.foreground}"`) %}
foreground="#ebdbb2"
```

Yolk will now be able to run the corresponding modifications on the file itself, allowing you to set
templated values while keeping the template directly in the same file.

### User Configuration
Yolk template expressions and configuration are written in the [Luau](https://luau.org/) scripting language.
You can provide custom data to use in your templates via the `yolk.lua` file in your yolk directory,
which allows you to fetch data dynamically from your system, or reference different static data depending on your system.

### Version Control
How does this work with version control?
Given that the concrete files in use on your system may be different across machines,
adding those to version control directly would result in a lot of merge conflicts frequently.
Yolk solves this by only commiting a "canonical" version of your templates, generated right before you commit.
This means that the version of your configuration seen in git will always be generated from a consistent, stable context.
