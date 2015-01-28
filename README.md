Rust GameBoy emulator (RGB)
===========================
This is a LR35902 (modified Z80 used in GameBoy) emulator written in Rust.

Dependencies
------------
In order to build `rgb`, you will need the latest version of Rust (www.rust-lang.org).
You can grab it with:

    $ curl -s https://static.rust-lang.org/rustup.sh | sudo sh

Then:

    $ git clone https://github.com/newbiz/rgb
    $ cd rgb
    $ make check                                # <=> cargo test
    $ make                                      # <=> cargo build
    $ ./target/rgbdbg                           # Debugger
    $ ./target/rgbemu                           # Emulator
    $ ./target/rgbas                            # Assembler
    $ ./target/rgbld                            # Linker

Authors
-------
- Aurelien Vallee <vallee.aurelien@gmail.com>

Features & Status
-----------------

- About a half of Z80 instruction set implemented
- No timers, no interrupts, no video, no cartridge support; just bare metal Z80.

Usage
-----
Example `rgbdbg` session with a simple Z80 program computing the
sum of integers from 1 to 10 in register `a`.

    RGB debugger 0.0.1
    Copyright (C) 2015 Aurelien Vallee
    License MIT: http://opensource.org/licenses/MIT
    (rgbdbg) Commands:
      help |h    List available commands
      file |f    Load Z80 executable binary
      next |n    Step execute the next instruction
      run  |r    Run the program until Cpu is stopped
      list |l    List upcoming instructions
      dump |d    Dump memory at location
      print|p    Print the current CPU state
      quit |q    Quit
    (rgbdbg) -> NOP
       NOP
       NOP
       NOP
       NOP
       NOP
       NOP
       NOP
       NOP
       NOP
    (rgbdbg) @x0000 | x00 x00 x00 x00 |
    @x0004 | x00 x00 x00 x00 |
    @x0008 | x00 x00 x00 x00 |
    @x000C | x00 x00 x00 x00 |
    @x0010 | x00 x00 x00 x00 |
    @x0014 | x00 x00 x00 x00 |
    (rgbdbg) |  A   F  |  B   C  |  D   E  |  H   L  |     PC |     SP | ZNHC---- |  M  |  T  |
    | x00 x00 | x00 x00 | x00 x00 | x00 x00 | @x0000 | @x0000 | 00000000 |   0 |   0 |
    |   0   0 |   0   0 |   0   0 |   0   0 |      - |      - |
    |       0 |       0 |       0 |       0 | @    0 | @    0 |
    (rgbdbg) Loading: samples/sum_integers.z80.
    Successfully loaded 8 bytes in memory.
    (rgbdbg) -> LD B, 0x0A
       XOR A
       ADD A, B
       DEC B
       JR NZ, 0xFE
       STOP 0
       NOP
       NOP
       NOP
       NOP
    (rgbdbg) @x0000 | x06 x0A xAF x80 |
    @x0004 | x05 x20 xFE x10 |
    @x0008 | x00 x00 x00 x00 |
    @x000C | x00 x00 x00 x00 |
    @x0010 | x00 x00 x00 x00 |
    @x0014 | x00 x00 x00 x00 |
    (rgbdbg) |  A   F  |  B   C  |  D   E  |  H   L  |     PC |     SP | ZNHC---- |  M  |  T  |
    | x00 x00 | x00 x00 | x00 x00 | x00 x00 | @x0000 | @x0000 | 00000000 |   0 |   0 |
    |   0   0 |   0   0 |   0   0 |   0   0 |      - |      - |
    |       0 |       0 |       0 |       0 | @    0 | @    0 |
    (rgbdbg) LD B, 0x0A
    (rgbdbg) XOR A
    (rgbdbg) ADD A, B
    (rgbdbg) DEC B
    (rgbdbg) JR NZ, 0xFE
    (rgbdbg) |  A   F  |  B   C  |  D   E  |  H   L  |     PC |     SP | ZNHC---- |  M  |  T  |
    | x0A x40 | x09 x00 | x00 x00 | x00 x00 | @x0003 | @x0000 | 01000000 |   8 |  32 |
    |  10  64 |   9   0 |   0   0 |   0   0 |      - |      - |
    |    2624 |    2304 |       0 |       0 | @    3 | @    0 |
    (rgbdbg) LD B, 0x0A
    XOR A
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    ADD A, B
    DEC B
    JR NZ, 0xFE
    STOP 0
    (rgbdbg) |  A   F  |  B   C  |  D   E  |  H   L  |     PC |     SP | ZNHC---- |  M  |  T  |
    | x37 xC0 | x00 x00 | x00 x00 | x00 x00 | @x0008 | @x0000 | 11000000 |  53 | 212 |
    |  55 192 |   0   0 |   0   0 |   0   0 |      - |      - |
    |   14272 |       0 |       0 |       0 | @    8 | @    0 |
    (rgbdbg) -> NOP
       NOP
       NOP
       NOP
       NOP
       NOP
       NOP
       NOP
       NOP
       NOP
    (rgbdbg) Bye.

Licensed under MIT/X
--------------------
Copyright (C) 2014 Aurelien Vallee

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
