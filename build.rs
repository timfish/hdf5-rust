fn main() {
    for (key, _) in std::env::vars() {
        if key == "DEP_HDF5_HAVE_DIRECT" {
            println!("cargo:rustc-cfg=h5_have_direct");
        }
        if key == "DEP_HDF5_HAVE_STDBOOL" {
            println!("cargo:rustc-cfg=h5_have_stdbool");
        }
        if key == "DEP_HDF5_HAVE_PARALLEL" {
            println!("cargo:rustc-cfg=h5_have_parallel");
        }
        if key == "DEP_HDF5_HAVE_THREADSAFE" {
            println!("cargo:rustc-cfg=h5_have_threadsafe");
        }
        if key.starts_with("DEP_HDF5_VERSION_") {
            let version = key.trim_start_matches("DEP_HDF5_VERSION_");
            println!("cargo:rustc-cfg=hdf5_{}", version);
        }
    }
}
