# oi

[![crates.io](https://img.shields.io/crates/v/oi.svg)](https://crates.io/crates/oi) [![docs](https://docs.rs/oi/badge.svg)](http://docs.rs/oi)

`oi` provides a location-annotated error type so you can display useful error messages to your users.

Error messages without location information are often not actionable, especially when they come from a complex program with many potential sources of errors.

Compare an unannotated error:

```sh
$ foo
No such file or directory (os error 2)
```

…with an annotated error:

```sh
$ foo
Configuration.toml: No such file or directory (os error 2)
```

`oi` is named after the exclamation, as in [Oi! Oi! Oi!](https://youtu.be/XWLU76o5rEI). Imagine alerting your users to the location of errors: "Oi! 1.2.3.4 is unreachable!"
