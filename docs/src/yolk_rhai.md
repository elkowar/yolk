# The yolk.rhai file

The `yolk.rhai` file is the heart of your Yolk configuration.
It's where you define all of your eggs (packages), as well as all the variables and functionality you will then refer to inside your templates.

## Basic structure

The `yolk.rhai` file is a Rhai script that is run by Yolk to generate your configuration.
Everything you declare in `yolk.rhai` will be available to use in your templates.

Your `yolk.rhai` needs to define one special variable called `eggs`.
This is a map of all the [eggs](./eggs.md) you have in your egg directory,
which describes where their files should be deployed to,
and which files should be treated as template files.

Let's look at an example:

```rust,ignore
let eggs = #{
    foot: #{
        targets: "~/.config/foot",
        templates: ["foot.ini"],
    },
    zsh: #{
        targets: #{
            ".zshrc": "~/.config/zsh/.zshrc",
            ".zshenv": "~/.zshenv",
        },
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
}
```

now, let's break this down.
For every entry in our `~/.config/yolk/eggs` directory, we have a corresponding egg configuration here in the eggs object.
This configuration can either just be the path where the eggs files should be deployed,
or an object.

If it's an object, it can contain the following fields:

#### `targets`
Either the path where to deploy the egg, or an object with mappings from file inside the egg directory to the target path.

Providing the string `"~/.config/foot"` is a short for `#{ ".": "~/.config/foot"}`.

Note that these directories get merged recursively.
This means, if you want to use a stow-style approach, and have the egg directory mirror your home directory structure, you can use
`"~"` (or `#{".": "~"}`) as the targets value.

#### `templates`
A list of files that should be treated as templates.
This list can contain shell-style glob patterns, so `*.lua` will expand to all lua files in the egg directory.
Files that are not listed here will not be edited by yolk during `yolk sync`!

#### `enabled`
a boolean, describing whether this egg should be deployed or not.
This is useful if you only want to deploy an egg on some systems, or depending on some other condition.





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
