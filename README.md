# num-conv
simple number converter for dec, hex, oct and binary

## Usage
just input the numbers as arguments, e.g.:

`clnc 123 0x2f 0o37 0b1100101` produces
```
 Decimal | Hexadecimal | Octal |  Binary
---------+-------------+-------+-----------
     123 |        0x7B | 0o173 | 0b1111011
      47 |        0x2F |  0o57 |  0b101111
      31 |        0x1F |  0o37 |   0b11111
     101 |        0x65 | 0o145 | 0b1100101
```

## Build
```bash
cargo build --release
```