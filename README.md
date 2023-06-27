
This repository configures the `comment_width` option of `rustfmt` to be 120 columns.
However, in certain contexts, `rustfmt` seems to wrap comments after only 119 columns.

Doc comments on functions can up to the full 120 columns (as expected).
But a doc comment on an enum variant can only go up to 119 columns.

To see this, run `cargo +nightly fmt --check`.
You'll see that it tries to wrap the 120 column long comment on the enum variant.
But the 120 column long comment on the function, and the 119 column long comment are fine.
