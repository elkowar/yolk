# Command-Line Help for `yolk`

This document contains the help content for the `yolk` command-line program.

**Command Overview:**

* [`yolk`](#yolk)
* [`yolk init`](#yolk-init)
* [`yolk status`](#yolk-status)
* [`yolk safeguard`](#yolk-safeguard)
* [`yolk adopt`](#yolk-adopt)
* [`yolk exec-canonical`](#yolk-exec-canonical)
* [`yolk eval`](#yolk-eval)
* [`yolk sync`](#yolk-sync)
* [`yolk eval-template`](#yolk-eval-template)
* [`yolk list`](#yolk-list)
* [`yolk edit`](#yolk-edit)
* [`yolk watch`](#yolk-watch)
* [`yolk git`](#yolk-git)

## `yolk`

Templated dotfile management without template files

**Usage:** `yolk [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `init` — Initialize the yolk directory
* `status` — Show the current state of your yolk eggs
* `safeguard` — Make sure you don't accidentally commit your local egg states
* `adopt` — Migrate a directory into an egg inside your yolk repository, and guide you through adding it to your yolk.rhai configuration
* `exec-canonical` — Run a given shell command within a canonical context. I.e.: `yolk exec-canonical gitui`
* `eval` — Evaluate a Rhai expression
* `sync` — Sync all template files and sync the deployments to match the configuration in yolk.rhai
* `eval-template` — Evaluate a given templated file, or read a templated string from stdin
* `list` — List all the eggs in your yolk directory
* `edit` — Open your `yolk.rhai` or the given egg in your `$EDITOR` of choice
* `watch` — Watch for changes in your templated files and re-sync them when they change
* `git` — Run a git-command within the yolk directory

###### **Options:**

* `--yolk-dir <YOLK_DIR>` — Provide a custom yolk directory
* `--home-dir <HOME_DIR>` — Provide a custom home directory that everything will be resolved relative to
* `-v`, `--debug` — Enable debug logging
* `--tracing-tree` — Enable displaying logs as a tree



## `yolk init`

Initialize the yolk directory.

This creates a directory called `yolk` within your config directory, and initializes it with the basic yolk directory structure.

**Usage:** `yolk init`



## `yolk status`

Show the current state of your yolk eggs

**Usage:** `yolk status`



## `yolk safeguard`

Make sure you don't accidentally commit your local egg states.

This renames `.git` to `.yolk_git` to ensure that git interaction happens through the yolk CLI

**Usage:** `yolk safeguard`



## `yolk adopt`

Migrate a directory into an egg inside your yolk repository, and guide you through adding it to your yolk.rhai configuration

**Usage:** `yolk adopt [OPTIONS] <EGG_NAME> <PATH>`

###### **Arguments:**

* `<EGG_NAME>` — Name of the egg that will be created
* `<PATH>` — Path that should be adopted into yolk

###### **Options:**

* `--strategy <STRATEGY>` — Whether this should be deployed via `merge` or `put` strategy. Defaults to `put`
* `--append-config-and-sync` — Append the generated config to yolk.rhai and sync the adopted egg without prompting
* `--templates <TEMPLATES>` — List of filepaths that should be included in the list of templated files



## `yolk exec-canonical`

Run a given shell command within a canonical context. I.e.: `yolk exec-canonical gitui`

**Usage:** `yolk exec-canonical [-- <COMMAND>...]`

###### **Arguments:**

* `<COMMAND>`



## `yolk eval`

Evaluate a Rhai expression.

The expression is executed in the same scope that template tag expressions are evaluated in.

**Usage:** `yolk eval [OPTIONS] <EXPR>`

###### **Arguments:**

* `<EXPR>` — The rhai expression to evaluate

###### **Options:**

* `--canonical` — Evaluate in canonical context instead



## `yolk sync`

Sync all template files and sync the deployments to match the configuration in yolk.rhai.

This will modify your template files in place, as well as deploying or undeploying any eggs to match your egg configuration.

**Usage:** `yolk sync [OPTIONS]`

###### **Options:**

* `--canonical` — Sync to canonical state. This should only be necessary for debugging purposes



## `yolk eval-template`

Evaluate a given templated file, or read a templated string from stdin

**Usage:** `yolk eval-template [OPTIONS] [PATH]`

###### **Arguments:**

* `<PATH>` — The path to the file you want to evaluate If not provided, the program will read from stdin

###### **Options:**

* `--canonical`



## `yolk list`

List all the eggs in your yolk directory

**Usage:** `yolk list`



## `yolk edit`

Open your `yolk.rhai` or the given egg in your `$EDITOR` of choice

**Usage:** `yolk edit [EGG]`

###### **Arguments:**

* `<EGG>`



## `yolk watch`

Watch for changes in your templated files and re-sync them when they change

**Usage:** `yolk watch [OPTIONS]`

###### **Options:**

* `--canonical`
* `--no-sync` — Don't actually update any files, just evaluate the templates and print any errors



## `yolk git`

Run a git-command within the yolk directory

**Usage:** `yolk git [OPTIONS] [COMMAND]...`

###### **Arguments:**

* `<COMMAND>`

###### **Options:**

* `--force-canonical` — Force yolk to run the command with canonicalized files, regardless of what command it is



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
