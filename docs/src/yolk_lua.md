# The yolk.lua file

The `yolk.lua` file is the heart of your Yolk configuration.
It's where you define all of the variables and functionality you will then refer to inside your templates.

## Basic structure

The `yolk.lua` file is a Lua script that is run by Yolk to generate your configuration.
It needs to contain at least two functions: `canonical_data()` and `local_data(system)`.

Both of these return a table, which is then used by Yolk to generate your configuration.
Inside your templates, yolk will make the table available under the global variable `data`.
If yolk is currently generating the config for your local system, the `local_data` will be used.

If yolk is evaluating the templates for use in version control, it will instead use the `canonical_data` function, which should return data that is fully stable across all systems.
This allows yolk to keep a stable, consistent state in version control, while having a dynamic, system-specific state on your local machine.
