mod bindings;

pub fn hello_world() {
    unsafe { bindings::hello_world() };
}

pub fn square(input: i32) -> i32 {
    unsafe { bindings::square(input) }
}
