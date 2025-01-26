# The yolk.rhai file

The `yolk.rhai` file is the heart of your Yolk configuration.

It's where you define all of your [eggs](eggs.md) (packages), as well as export any variables and functionality you will then refer to inside your [templates](templates.md).

If you're familiar with [Rhai](https://rhai.rs/), this is loaded as a module, imported into the global scope of your template tags.

## Basic structure

The `yolk.rhai` file is a Rhai script that is run by Yolk to generate your configuration.
Everything you declare in `yolk.rhai` will be available to use in your templates.

Your `yolk.rhai` needs to define one special variable called `eggs`.
This is a map of all the [eggs](./eggs.md) you have in your egg directory,
which describes where their files should be deployed to,
and which files should be treated as template files.

Let's look at an example:

```rust,ignore
export let eggs = #{
    foot: #{
        targets: "~/.config/foot",
        templates: ["foot.ini"],
    },
    zsh: #{
        targets: #{
            ".zshrc": "~/.config/zsh/.zshrc",
            ".zshenv": "~/.zshenv",
        },
        main_file: ".zshrc"
    },
    nvim: #{
        targets: "~/.config/nvim",
        // Note that you can use shell-style glob patterns for the templates list
        templates: ["**/*.lua"],
    },
    niri: #{
        targets: "~/.config/niri",
        templates: ["config.kdl"],
        enabled: SYSTEM.hostname == "cool-desktop",
    },
    alacritty: "~/.config/alacritty",
    // This uses the `merge` deployment strategy, which
    // will merge the directory structures during deployment,
    // allowing a stow-style pattern
    // of mirroring your home directory structure in your egg dir
    zed: { targets: "~", strategy: "merge" }
}
```

now, let's break this down.
For every entry in our `~/.config/yolk/eggs` directory, we have a corresponding egg configuration here in the eggs object.
This configuration can either just be the path where the eggs files should be deployed,
or an object.

If it's an object, it can contain the following fields:

#### `enabled`
a boolean, describing whether this egg should be deployed or not.
This is useful if you only want to deploy an egg on some systems, or depending on some other condition.

#### `targets`
Either the path where to deploy the egg, or an object with mappings from file inside the egg directory to the target path.

Providing the string `"~/.config/foot"` is a short for `#{ ".": "~/.config/foot"}`.

#### `strategy`
Either `"put"` or `"merge"`. Defaults to `put`.

- In **put** mode, yolk will create a symlink for each mapping from egg directory entry to target path.
If a directory or file already exists, Yolk will refuse to create the symlink.

- In **merge** mode, yolk will merge the directory structures during deployment.
This means, if you want to use a stow-style approach, and have the egg directory mirror your home directory structure, you can use
`"~"` (or `#{".": "~"}`) as the targets value.

#### `templates`
A list of files that should be treated as templates.
This list can contain shell-style glob patterns, so `*.lua` will expand to all lua files in the egg directory.
Files that are not listed here will not be edited by yolk during `yolk sync`!


#### `main_file`
A path, relative to the egg directory, that will be opened when you run `yolk edit <eggname>`.




## Available variables

To generate your configuration depending on your system, there are a couple global variables that you can reference inside the `yolk.rhai` file.
The `SYSTEM` variable is a table containing data about your local system.
If the config is being executed in canonical mode, the `SYSTEM` table will instead contain a fixed set of values that will be the same across all systems.

To know if you're currently in local or canonical mode, you can check the `LOCAL` variable.

**Tip:**
To look at the contents of those variables or try out your logic, you can always use the `yolk eval` command.

```bash
$ yolk eval 'print(SYSTEM)'
```

## Splitting up into multiple files

Rhai allows you to import other files into your scripts.
For example, let's say you want to keep your color theme definition in a separate file.
Simply create a new `colors.rhai` file next to your `yolk.rhai`, and make sure to explicitly declare exported variables as `export`:

```rust
export let gruvbox = #{
  background: "#282828",
  foreground: "#ebdbb2",
};

fn some_function() {
    print("hi")
}
```

Note that functions are exported by default.

Now, in your `yolk.rhai`, import this script, giving the module an explict name:

```rs
import "colors" as colors;
```

Now you can refer to anything exported from that file as `colors::thing`, i.e.:

```rs
let theme = colors::gruvbox;
colors::some_function();
```
