# rsd

`rsd` Rust implemention of something resembling `xxd -e -l 64`. Its functionality is limited to looking at the headers of ELF binaries and outputting the details in a mostly- human-readable format.

## Why?

I wanted to learn Rust a little better; I also wanted a more readable version of `xxd -e -l 64` when parsing ELF headers rather than parsing something like this:
```
ced27abc2bef:/# xxd -e -l 64 /bin/sh
00000000: 464c457f 00010102 00000000 00000000   .ELF............
00000010: 00b70003 00000001 0000a780 00000000   ................
00000020: 00000040 00000000 000a0348 00000000   @.......H.......
00000030: 00000000 00380040 00400009 00180019   ....@.8...@.....
```
or this:
```
ced27abc2bef:/# xxd -l 64 /bin/sh
00000000: 7f45 4c46 0201 0100 0000 0000 0000 0000  .ELF............
00000010: 0300 b700 0100 0000 80a7 0000 0000 0000  ................
00000020: 4000 0000 0000 0000 4803 0a00 0000 0000  @.......H.......
00000030: 0000 0000 4000 3800 0900 4000 1900 1800  ....@.8...@.....
```

I've been writing YARA rules recently and knowing how to locate information like this can prove useful, though making it more human-readable is more efficient as well.

## What works for now?

- Building locally via `rustc` or via Dockerfile
- ELF binaries

## Usage

`./rsd <program name>`

Example (run from Wolfi):
```
./rsd /bin/sh
Full header:
7F 45 4C 46 02 01 01 00 00 00 00 00 00 00 00 00 03 00 B7 00 01 00 00 00 80 A7 00 00 00 00 00 00 40 00 00 00 00 00 00 00 48 03 0A 00 00 00 00 00 00 00 00 00 40 00 38 00 09 00 40 00 19 00 18 00 ELF File Type: Shared (0x03)
Machine Type: AArch64 (0x00B7)

ELF Class: 64-bit
Data Encoding: Little-endian
ELF Version: 1
Entry Point Address: 42880
Program Header Table Offset: 64
Section Header Table Offset: 656200
ELF Header Size: 64 bytes
Program Header Table Entry Size: 56 bytes
Number of Program Header Table Entries: 9
Section Header Table Entry Size: 64 bytes
Number of Section Header Table Entries: 25
Section Header String Table Index: 24

Segment Information:
Segment 0:
  Type: PT_NOTE (0x00000004)
  Offset: 64
  Virtual Address: 64
  Physical Address: 64
  File Size: 0x00000000000001F8 (504 bytes)
  Memory Size: 0x00000000000001F8 (504 bytes)
  Flags: Unknown (0x00000008)

Segment 1:
  Type: PT_NOTE (0x00000004)
  Offset: 568
  Virtual Address: 568
  Physical Address: 568
  File Size: 0x000000000000001B (27 bytes)
  Memory Size: 0x000000000000001B (27 bytes)
  Flags: R (0x00000001)

Segment 2:
  Type: PT_SHLIB (0x00000005)
  Offset: 0
  Virtual Address: 0
  Physical Address: 0
  File Size: 0x000000000008F118 (586008 bytes)
  Memory Size: 0x000000000008F118 (586008 bytes)
  Flags: Unknown (0x00010000)

Segment 3:
  Type: PT_PHDR (0x00000006)
  Offset: 646864
  Virtual Address: 646864
  Physical Address: 646864
  File Size: 0x0000000000002399 (9113 bytes)
  Memory Size: 0x0000000000002A00 (10752 bytes)
  Flags: Unknown (0x00010000)

Segment 4:
  Type: PT_PHDR (0x00000006)
  Offset: 651720
  Virtual Address: 651720
  Physical Address: 651720
  File Size: 0x0000000000000220 (544 bytes)
  Memory Size: 0x0000000000000220 (544 bytes)
  Flags: Unknown (0x00000008)

Segment 5:
  Type: PT_NOTE (0x00000004)
  Offset: 596
  Virtual Address: 596
  Physical Address: 596
  File Size: 0x0000000000000020 (32 bytes)
  Memory Size: 0x0000000000000020 (32 bytes)
  Flags: X (0x00000004)

Segment 6:
  Type: PT_NOTE (0x00000004)
  Offset: 585820
  Virtual Address: 585820
  Physical Address: 585820
  File Size: 0x0000000000000034 (52 bytes)
  Memory Size: 0x0000000000000034 (52 bytes)
  Flags: X (0x00000004)

Segment 7:
  Type: PT_PHDR (0x00000006)
  Offset: 0
  Virtual Address: 0
  Physical Address: 0
  File Size: 0x0000000000000000 (0 bytes)
  Memory Size: 0x0000000000000000 (0 bytes)
  Flags: Unknown (0x00000010)

Segment 8:
  Type: PT_NOTE (0x00000004)
  Offset: 646864
  Virtual Address: 646864
  Physical Address: 646864
  File Size: 0x0000000000002130 (8496 bytes)
  Memory Size: 0x0000000000002130 (8496 bytes)
  Flags: R (0x00000001)
```

Running `rsd` from MacOS will result in this:
```
❯ ./rsd /bin/sh
/bin/sh is not an ELF file (CAFEBABE).
```

## Will anything be added to this project?

Maybe -- I want to start automating various things in whatever language seems right. 

Analyizing Mach-O binaries (i.e., MacOS binaries) doesn't seem to be as common but would be something to support in a future PR.
