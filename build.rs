fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Specify the extra ESP-IDF component (e.g., onewire_bus)
    println!("cargo:esp-idf-sys=extra_components_remote=name=onewire_bus,version=^1.0.2");
    embuild::espidf::sysenv::relay();
    embuild::espidf::sysenv::output(); // Only necessary for building the examples
}