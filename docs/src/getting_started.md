# Getting started

**Remember: Yolk is currently in very early development. Expect breakages, lost files, and other issues.
Always have a good backup of your files before using Yolk in this stage. You have been warned.**

Yolk uses an approach to dotfile management very similar to [GNU Stow](https://www.gnu.org/software/stow/):
Your dotfiles are stored in a separate directory (typically `~/.config/yolk/eggs/<egg-name>/<path_to_your_configs>`),
and are then symlinked into their original location.

Yolk groups dotfiles into so-called "eggs", which are packages of configuration,
typically for one single application (although you can group them however you want, or even just have one egg for all your configuration files).

Inside an egg directory, yolk mirrors the directory structure of your home directory, to ensure that it knows where to place your files.

## Initial setup

To get started with Yolk, you'll first need to set up the Yolk file structure.

```bash
$ yolk init
```

This will create the yolk directory, with a default `yolk.lua` file, and an `eggs` directory.

### Adding your first egg

let's say we want to manage the configuration for the `alacritty` terminal emulator.
To do this, we add it to a new egg called `alacritty`:

```bash
$ yolk add alacritty ~/.config/alacritty
```

Let's look at what just happened:

- Yolk created a new directory `~/.config/yolk/eggs/alacritty`. This is called the egg directory.
- Inside that directory, Yolk set up a file structure mirroring your home directory,
  and moved the provided alacritty config dir into there: `~/.config/yolk/eggs/alacritty/.config/alacritty`.
- Yolk set up a symlink from that directory back to the original location: `~/.config/alacritty`.

### Committing your dots to git

Now, we want to make sure our dotfiles are in version control and pushed to our git host of choice.
Every interaction with git should be done through the `yolk git` command.
This ensures that git sees the canonical (stable) representation of your files, and automatically performs them from within the yolk directory.

```bash
$ yolk git init
$ yolk git add --all
$ yolk git commit -m "Setup alacritty"
```

You can now set up your git reomte and use git as usual -- just remember to always use `yolk git`, especially when you're committing your files.

### Baby's first template

Because you too are very indecisive about your terminal colors,
you now decide you want to use yolk to manage your color theme for alacritty, and any other applications that you might add later.
You also decide that you want to use a different color scheme on your desktop and your laptop.

To achieve this, let's first declare your color theme in your `~/.config/yolk/yolk.lua` file:

```lua
data = {
  colors = if SYSTEM.hostname == "laptop" then {
    background = "#282828",
    foreground = "#ebdbb2",
  } else {
    background = "#000000",
    foreground = "#ffffff",
  }
}
```

Beautiful!
What we're doing here is setting up a table called `data`, which will store any user-data we might want to refer to in our templates in the future.
We set up a field `colors`, which we then set to a different color scheme depending on the hostname of the system.

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

Of course, we now need to apply this template.
However, running `yolk sync` will not do anything just yet: We first need to tell Yolk about the fact that this file contains a template.
To do this we run the following:

```bash
$ yolk make-template ~/.config/alacritty/alacritty.yml
```

This will now add the `alacritty.toml` file into a special `yolk_templates` file inside the alacritty egg.
Now, run:

```bash
$ yolk sync
```

You will see that, your `alacritty.toml` has changed, and the colors from your `yolk.lua` file have been applied, depending on your hostname.
