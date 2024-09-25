# rust-hexdump: hexdump command written in Rust! :crab:
### "display file contents in hexadecimal, decimal, octal, or ascii" - [linux manpage](https://www.man7.org/linux/man-pages/man1/hexdump.1.html)

## Usage
To run, provide the filepath and any options that you would like!
Only a primitive set of options from `hexdump` are are provided, in particular:
```
-n, --length length
    Interpret only length bytes of input.
-x, --two-bytes-hex
    Two-byte hexadecimal display. Display the input offset in
    hexadecimal, followed by eight space-separated, four-column,
    zero-filled, two-byte quantities of input data, in
    hexadecimal, per line.
-d, --two-bytes-decimal
    Two-byte decimal display. Display the input offset in
    hexadecimal, followed by eight space-separated, five-column,
    zero-filled, two-byte units of input data, in unsigned
    decimal, per line.
-o, --two-bytes-octal
    Two-byte octal display. Display the input offset in
    hexadecimal, followed by eight space-separated, six-column,
    zero-filled, two-byte quantities of input data, in octal, per
    line.
-b, --one-byte-octal
    One-byte octal display. Display the input offset in
    hexadecimal, followed by sixteen space-separated,
    three-column, zero-filled bytes of input data, in octal, per
    line.
-c, --one-byte-char
    One-byte character display. Display the input offset in
    hexadecimal, followed by sixteen space-separated,
    three-column, space-filled characters of input data per line.
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

## Implementation Notes
Some cool functionality that I wanted to feature in this implementation was the fact that you could print the hexadecimal, decimal, octal, or ascii representations of the file's bytes simultaneously, and in a specific noted order. For example:
```bash
❯ hexdump -b -x -c -n 32 examples/html.txt
0000000 131 157 165 040 143 141 156 047 164 040 160 141 162 163 145 040
0000000    6f59    2075    6163    276e    2074    6170    7372    2065
0000000   Y   o   u       c   a   n   '   t       p   a   r   s   e
0000010 133 130 135 110 124 115 114 040 167 151 164 150 040 162 145 147
0000010    585b    485d    4d54    204c    6977    6874    7220    6765
0000010   [   X   ]   H   T   M   L       w   i   t   h       r   e   g
0000020
```
At this point in time, I realized that I didn't have a way to retain the order of the options given with `clap`, so I ended up additionally using the Rust standard library to fetch and handle certain option flags while retaining their order, like `-b`, manually. The `clap` `Args` struct remains as to at least verify the syntax of the arguments passed into the CLI tool--it parses flags like `-b` but doesn't actually do anything with them. My implementation handles the above case by creating an iterator struct for each of the option flags given, namely ones like `-b` `-x` `-c`, and storing them in an array. Each of the iterators in my array is called in order to print each representation. A "fun" thing to note is that the same option flag can be used multiple times, for example `-b -b`, in which the octal representation would be doubly printed. Though all two-byte printing styles have practically the same semantics, for example `TwoByteHexadecimal` and `TwoByteDecimal`, and could be kept within one unified `TwoByte` struct, I chose to divide them into their own structs as to avoid an O(1) check within `TwoByte`'s `next()` function when determining which numeric formatting to use. I based the output formatting from what I saw on my Macbook, so the output may differ from the actual `hexdump` command depending on the machine you're running it on.

## Credits
The functionality of this tool is meant to mimic [`hexdump`](https://www.man7.org/linux/man-pages/man1/hexdump.1.html). Hence, many of the options and their explanations are taken directly from the manpage. In addition, `examples/html.txt` is a copy-paste of [this notorious StackOverflow post](https://stackoverflow.com/questions/1732348/regex-match-open-tags-except-xhtml-self-contained-tags#:~:text=While%20arbitrary%20HTML%20with%20only%20a%20regex%20is) and `examples/bee_movie.txt` is a copy-paste the "Bee Movie (2007)" script. All credits go to the original authors of these works.