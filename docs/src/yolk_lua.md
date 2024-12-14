# The yolk.luau file

The `yolk.luau` file is the heart of your Yolk configuration.
It's where you define all of the variables and functionality you will then refer to inside your templates.

## Basic structure

The `yolk.luau` file is a Lua script that is run by Yolk to generate your configuration.
Everything you declare in `yolk.luau` will be available to use in your templates.

To generate your configuration depending on your system, there are a couple global variables that you can reference inside the `yolk.luau` file.
The `SYSTEM` variable is a table containing data about your local system.
If the config is being executed in canonical mode, the `SYSTEM` table will instead contain a fixed set of values that will be the same across all systems.

To know if you're currently in local or canonical mode, you can check the `LOCAL` variable.

**Tip:**
To look at the contents of those variables or try out your logic, you can always use the `yolk eval` command.

```bash
$ yolk eval 'inspect.inspect(SYSTEM)'
```
