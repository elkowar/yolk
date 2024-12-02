# Eggs

An egg is one package of configuration, typically for one single application.
For example, your editor configuration `~/.config/nvim` would likely be one egg called `nvim`.

When you want to deploy a specific application configuration to your system, you deploy the corresponding egg using `yolk use`.
You can create a new egg using `yolk add`, which will move the given paths into the `eggs` directory,
create a new egg for you, and set up a symlink back to the original file location.
