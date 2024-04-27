# rusty-wc

I like rusty spoons!

## What's the exercise?

Just learn some rust by practicing a "real-life" example of adding a feature to
this CLI. The CLI we're basing off of is Good ol' `wc`.

The feature you'll implement is adding a `-f` flag - which will count frequency of
words in the input files, and print the top 10 most frequent words.

### Bonus points!

Implement it with parallelism, and add a benchmark to compare the performance
of the parallel implementation with the sequential one. Utilize max available
cores for parallelism.

## Installation

Follow the [Rust installation guide](https://www.rust-lang.org/tools/install).

The exercise was written with rustc `1.79.0-nightly`. Check yours with `rustc --version` and if needed, run:

```sh
rustup update
```

### IDEs

In VSCode, install the
[Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
extension, and make sure to install a debugger as well.

For JetBrains IDEs, try [RustRover](https://www.jetbrains.com/rust/).
