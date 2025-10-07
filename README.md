# Wild test files

This repository contains binary files used by some of Wild's tests. It's
separate so that if it gets too large, we can prune commits or otherwise
rewrite history in order to reduce its size, things that we definitely don't
want to do with the main wild repository.

## Provenance and self-check
Every binary checked into this repo must have a corresponding entry in `Manifest.toml`.
Each entry records the binary's hash, name and license.

The `selfcheck` directory has a small program in it that will ensure that the hashes match those specified and that the licenses are allowed.
No binaries in this repository are used to produce Wild.
They are not used to create derived works.
They are copies placed in this repository for convenient access.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT)
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
Wild by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

Note that most of the content in this repository isn't under these licenses.
See details in `Manifest.toml` for the licenses on those files.
