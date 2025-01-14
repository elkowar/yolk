
# Getting started

<div class="warning">

**Remember: Always have a good backup of your files before using any tool that modifies your files. Nothing bad should happen here, but better be careful.**

</div>

## How dotfiles are stored

Chicken manages your dotfiles by storing them in a separate directory, typically inside `~/.config/chicken`.
This allows you to keep your dotfiles in version control easily, and lets you manage your configuration from one central location.

Chicken groups dotfiles into so-called ["eggs"](eggs.md), which are packages of configuration,
typically for one single application (although you can group them however you want, or even just have one egg for all your configuration files).

When an egg "hatches" (is deployed), Chicken creates symlinks in the target location pointing towards the egg directory.
This way, the configured appliactions will see the configuration files as they expect to see them.

To define where a set of configuration files should be deployed to, you declare each of your eggs in your [main chicken configuration file](./chicken_rhai.md).
This allows you, among other things, to define a different target directory per system.

## Initial setup

To get started with Chicken, you'll first need to set up the Chicken file structure.

```bash
$ chicken init
```

This will create the chicken directory, with a default `chicken.rhai` file, and an `eggs` directory.

### Adding your first egg

let's say we want to manage the configuration for the `alacritty` terminal emulator.
To do this, we first move our alacritty configuration into the `eggs` directory:

```bash
$ mv ~/.config/alacritty ~/.config/chicken/eggs/
```

And then configure the corresponding [egg deployment](./chicken_rhai.md#basic-structure):

```rust,ignore
export let eggs = #{
    alacritty: #{
        targets: "~/.config/alacritty",
        templates: ["alacritty.yml"],
        enabled: true,
    }
}
```

Now we can run `chicken sync`!
This will set up a symlink from the target location `~/.config/alacritty`
back to the alacritty egg directory `~/.config/chicken/eggs/alacritty`.

### Committing your dots to git

Now, we want to make sure our dotfiles are in version control and pushed to our git host of choice.
Every interaction with git should be done through the `chicken git` command.
This ensures that git sees the canonical (stable) representation of your files, and automatically performs them from within the chicken directory.

```bash
$ chicken git init
$ chicken safeguard
$ chicken git add --all
$ chicken git commit -m "Setup alacritty"
```

To understand what `chicken safeguard` does, see [safeguarding git](./git_concepts.md#safeguarding-git).

You can now set up your git reomte and use git as usual -- just remember to always use `chicken git`, especially when you're committing your files.

### Baby's first template

Because you too are very indecisive about your terminal colors,
you now decide you want to use chicken to manage your color theme for alacritty, and any other applications that you might add later.
You also decide that you want to use a different color scheme on your desktop and your laptop.

To achieve this, let's first add a declaration of your color theme in your `~/.config/chicken/chicken.rhai` file:

```rust,ignore
// ... snip ...

const themes = #{
    gruvbox: #{
        background: "#282828",
        foreground: "#ebdbb2",
    },
    mono: #{
        background: "#000000",
        foreground: "#ffffff",
    },
}

export const data = #{
    colors = if SYSTEM.hostname == "laptop" { themes.gruvbox } else { themes.mono }
}
```

Beautiful!
What we're doing here is setting up an *exported* table called `data`, which will store any user-data we might want to refer to in our templates in the future.
We set up a field `colors`, which we then set to a different color scheme depending on the hostname of the system.

**Don't forget to `export` any variables you might want to reference in your template tags!**

Now, let's set up a template in our alacritty config file:

```toml
#...
[colors.primary]
background = "#ff0000" # {< replace_color(data.colors.background) >}
foreground = "#ff0000" # {< replace_color(data.colors.foreground) >}
# ...
```

Let's break down what's happening here:
Inside the comments after the color declarations, we're using "inline template tags", as indicated by the `{< ... >}` syntax.
These inline tags transform whatever is before them in the line.
The tag calls the built-in `replace_color` function, which looks for a hex-code and replaces it with the value from the `data.colors` table.

**Let's try it**!
Run

```bash
$ chicken sync
```

You will see that, your `alacritty.toml` has changed, and the colors from your `chicken.rhai` file have been applied, depending on your hostname.
