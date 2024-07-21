fn main() {
    cc::Build::new()
        .file("c-toxcore/toxcore.c")
        .compile("toxcore");
}
