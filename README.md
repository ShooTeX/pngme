# PNGme

> Small CLI tool to hide secret messages inside PNG files

This project was made in the course of reading the
[PNGme](https://picklenerd.github.io/pngme_book/) book.

```stdout
Simple CLI tool to hide messages inside a PNG

Usage: pngme <COMMAND>

Commands:
  encode  Encode a secret message into a PNG file
  decode  Decode a secret message from a PNG file
  remove  Remove chunk from PNG
  print   Print from PNG
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

```stdout
Encode a secret message into a PNG file

Usage: pngme encode <INPUT> <CHUNK_TYPE> <MESSAGE> [OUTPUT_PATH]

Arguments:
  <INPUT>        File path or url to a png file
  <CHUNK_TYPE>   A chunk type, i.e. `ruSt`
  <MESSAGE>      Your secret message
  [OUTPUT_PATH]  The output for the PNG with the secret message

Options:
  -h, --help  Print help information
```
