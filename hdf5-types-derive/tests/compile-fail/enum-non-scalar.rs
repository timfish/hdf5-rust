#[macro_use]
extern crate hdf5_types_derive;

#[derive(H5Type)]
//~^ ERROR proc-macro derive
//~^^ HELP H5Type can only be derived for enums with scalar discriminants
enum Foo {
    Bar
}
