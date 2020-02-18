# hdf5-sys

This crate provides bindings to the c library `hdf5`.

## Features

Most features will be autodetected at compile time. Additionally, some features may be selected through `features`. These are not currently respected when not using the `static` flag.

* `mpio` : MPI support
* `hl` : Higher level library bindings
* `threadsafe` : Requests a thread-safe version of `hdf5`
* `zlib` : `zlib` filter support
* `deprecated` :  Includes deprecated symbols
* `static` : Compiles `hdf5` from source. Be aware that `hdf5` has separate licensing (modified BSD 3-Clause)
