fn main() {
    let tuple = (1, 500, false, "Hello world", 0.4);
    println!("{}", tuple.0); // Zero - Based

    let (a, b , c, d, e) = tuple;

    println!("{}", a); //1
    println!("{}", b); //500
    println!("{}", c); // ....
    println!("{}", d);
    println!("{}", e);
}
