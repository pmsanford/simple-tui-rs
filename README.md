> [!CAUTION]
> This repo uses outdated methods and does not make a good starting point for ratatui. It may be updated in the future, but until then, it should not be used. See [this message](https://github.com/ratatui-org/ratatui/pull/651#issuecomment-1827608876) for details.

# simple-tui-rs

An example app using [tui-rs](https://github.com/fdehau/tui-rs) and [termion](https://github.com/redox-os/termion) with non-blocking input. I created this because while the tui-rs examples are comprehensive, they're doing a lot, and it's difficult to use one of them as a starting point for your own app due to the common app and demo code they all utilize.

This app creates a tui-rs terminal, clears the screen, and changes the color of a single block in a loop while listening asynchronously (through termion's async_stdin) for `q` to quit.
