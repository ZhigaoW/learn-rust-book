fn main() {
    println!("{0}, {1}, {1}", "a", "b");
    

    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    #[allow(dead_code)]
    struct Structure(i32);


}
