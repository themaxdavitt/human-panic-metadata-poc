fn main() {
    wrapper::working_custom_panic_hook!();

    panic!("oops");
}
