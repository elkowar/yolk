# Yolk – Painfree Templated Dotfile Management

Yolk is a dotfile management tool with a unique spin,
sitting somewhere in between [GNU Stow](https://www.gnu.org/software/stow/) and [chezmoi](https://www.chezmoi.io/).

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
# {% replace /".*"/ `"${data.colors.background}"` %}
background="#282828"
# {% replace /".*"/ `"${data.colors.foreground}"` %}
foreground="#ebdbb2"
```
