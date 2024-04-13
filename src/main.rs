fn main() {
    #[cfg(target_arch = "xtensa")]
    esp_idf_sys::link_patches();
    let value = cpp_example::square(4);

    println!("the square of {} is {}", 4, value);
    cpp_example::hello_world();
}
