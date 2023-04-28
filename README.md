# ctp_check

check if catppuccin is ported to *insert app*.

# Usage

`ctp_check "name_of_app"` where name_of_app is the repo name.

# WHY should I use this?

You shouldn't. This app doesn't even have fuzzy search (yet).

# WHY did I make this?

I wanted to speedrun an app.

# Installation

## dependencies
 * make
 * the rust toolchain (cargo, rustc, etc.)

## build+install (most people should do this)

`make` or `make all`. This will add `$HOME/.local/bin` to `$PATH` in the current session.

## build and install only

this **wont** add `$HOME/.local/bin` to path:

`make build install`


# De-installation

`make uninstall`.
