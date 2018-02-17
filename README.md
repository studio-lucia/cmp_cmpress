# cmp_cmpress

`cmp_cmpress` is a commandline tool which compresses data using Sega's CMP [run-length encoding](http://en.wikipedia.org/wiki/Run-length_encoding) compression format, which was commonly used in Sega Saturn games. This tool aims to be 100% compatible with Sega's original decoder; you should be able to use its output to replace files in retail Saturn games.

This tool uses the [`sega_cmp`](https://github.com/studio-lucia/sega_cmp) Rust crate, which is based on [@MrConan1](https://github.com/MrConan1)'s C tool [CMP_CMPRESS](https://github.com/MrConan1/CMP_CMPRESS). This commandline tool reimplements all of that tool's options exactly. Aside from some minor differences in how the CLI help works, they should behave exactly the same.

## Installing

To install from source, just run `make install`; this will install the binary to `/usr/local`. You can also specify a different location to install to by specifying the `PREFIX` option, such as `make install PREFIX=/opt/local`.

## Contributing

1. Fork the repository
2. Create a new branch
3. Commit your changes
4. Open a pull request
5. 🎉

## Bugs and support

Please report any issues on this repository's issue tracker. I'll try to do whatever I can to help!
