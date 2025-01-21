# Git concepts

Basic familiarity with git is assumed.

## How yolk works with git

An important part of how yolk works is that your dotfiles will always be committed in a "canonical" state, no matter from what system.
Yolk achieves this by setting up a git filter that will automatically transform all templated files into their canonical variants whenever git is reading them.

## Cloning your dotfiles

To clone your dotfiles on a new machine, simply clone the repository to `.config/yolk`, and safeguard your git directory.

```bash
$ git clone <your-dots-repo> "$XDG_CONFIG_HOME/yolk"
```

After that, you can start `yolk sync`ing your eggs!

## Interacting with git

To stage or commit changes, get the git diff or status, you can use the `yolk git` command, which behaves just like the `git` CLI.
So, instead of

- `git status`, you run `yolk git status`,
- `git add .`, you run `yolk git add --all`,
- `git commit -m "cool changes"`, you run `yolk git commit -m "cool changes`,

and so on.
