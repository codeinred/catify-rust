# A simple project to prettify commit messages

*NOW IN RUST!*

Commit messages are good. They provide information about tons of things. But far
too many commit messages lack emojis! This is a critical problem, and one that I
will not stand for.

Catify solves this issue. In order to use catify, you can install it via cargo:

```bash
cargo install --path .
```

And then, you can create a link to whatever directory you've selected to store
your git hooks. This could mean adding it as a hook to a particular project:

```bash
ln $CARGO_HOME/bin/catify-rust .git/hooks/prepare-commit-message
```

Or placing it as a hook to a directory that stores global git hooks:

```bash
ln $CARGO_HOME/bin/catify-rust ~/.local/git-hooks/prepare-commit-message
```

Here, I've selected `~/.local/git-hooks` as my git hooks repository, however you
can choose whatever directory you'd like by configuring `core.hooksPath`:

```bash
git config --global core.hooksPath "$HOME/.local/git-hooks"
```

And you're done!

![Catify Example](/catify.png)

## Reasons to use catify

There are a number of reasons you'd like to use catify, but I've provided a
small sampling of them below.

- Using emojis in commits is neat
- It adds whimsy to an otherwise monotonous task
- It encourages better support for unicode
- Gnome terminal now supports emojis, so your commit messages will display
  properly
- And finally...

Catify is fast, and should have a negligible impact on the time to create a
commit.

I hope you enjoy your newly-emoted commit messages!
