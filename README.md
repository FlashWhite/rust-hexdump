# rust-hexdump: hexdump command written in Rust! :crab:
### "display file contents in hexadecimal, decimal, octal, or ascii" - [linux manpage](https://www.man7.org/linux/man-pages/man1/hexdump.1.html)

## Usage
To run, provide the filepath and any options that you would like!
Only a primitive set of options from `hexdump` are are provided, in particular:
```
-n, --length length
    Interpret only length bytes of input.
// TODO: 
-b, --one-byte-octal
    One-byte octal display. Display the input offset in
    hexadecimal, followed by sixteen space-separated,
    three-column, zero-filled bytes of input data, in octal, per
    line.
-c, --one-byte-char
    One-byte character display. Display the input offset in
    hexadecimal, followed by sixteen space-separated,
    three-column, space-filled characters of input data per line.
-d, --two-bytes-decimal
    Two-byte decimal display. Display the input offset in
    hexadecimal, followed by eight space-separated, five-column,
    zero-filled, two-byte units of input data, in unsigned
    decimal, per line.
```

Example Usage:
```bash
❯ cargo run -- -n 42 -f examples/html.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/rust-hexdump -n 42 -f html.txt`
0000000 6f59 2075 6163 276e 2074 6170 7372 2065
0000010 585b 485d 4d54 204c 6977 6874 7220 6765
0000020 7865 202e 6542 6163 7375
000002a
```

## Credits
The functionality of this tool is meant to mimic [`hexdump`](https://www.man7.org/linux/man-pages/man1/hexdump.1.html). Hence, many of the options and their explanations are taken directly from the manpage. In addition, `examples/bee_movie.txt` is a copy-paste the "Bee Movie (2007)" script. All credits go to the screenwriters.