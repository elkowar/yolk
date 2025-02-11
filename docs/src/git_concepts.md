# Git concepts

Basic familiarity with git is assumed.

## Safeguarding git

Yolk wraps the git CLI to ensure that git only ever interacts with your dotfiles in their canonical state.
If it didn't do that, you would end up committing the local state of your dotfiles,
which would conflict with their state from another machine -- which is what yolk is trying to solve.

To ensure that you're not accidentally using the regular git CLI for your dotfiles, it is recommended to "safeguard" your dotfiles' git directory.
To do this, simply run

```bash
$ yolk safeguard
```

after cloning or initializing your dotfiles.

This simply renames the `.git` directory to `.yolk_git`, which means the regular git CLI won't see the repository anymore.
You are now more or less forced to use the `yolk git` command instead -- which conveniently doesn't just ensure consistency of the git state,
but also works from anywhere in your filesystem!

## Cloning your dotfiles

To clone your dotfiles on a new machine, simply clone the repository to `.config/yolk`, and safeguard your git directory.

```bash
$ git clone <your-dots-repo> "$XDG_CONFIG_HOME/yolk"
$ yolk safeguard
```

After that, you can start `yolk sync`ing your eggs!

## Interacting with git

To stage or commit changes, get the git diff or status, you can use the `yolk git` command, which behaves just like the `git` CLI.
So, instead of

- `git status`, you run `yolk git status`,
- `git add .`, you run `yolk git add --all`,
- `git commit -m "cool changes"`, you run `yolk git commit -m "cool changes`,

and so on.
This ensures the files are always in the correct canonical state, and makes it possible to interact with a safeguarded git repository.
