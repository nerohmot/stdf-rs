# stdf-rs

stdf-rs is both a library and a set of cli tools that facilitates working
with STDF files in bash & Co.

The library also has a python API.

## stdf_endian

Given an STDF file, this tool will determine the used endian (BE or LE).
If the given STDF file exists, but doesn't contain a full FAR record, NA will be returned.
If the STDF file doesn't exist, the return code is none-zero.

```bash
$ stdf_endian --input 
```
