fn main() {
    let arr  = [1, 2, 3, 4]; //Im Gegensatz zu einem Tuple muss der Type in einem Array für jeden Wert der selbe sein
    println!("{}", arr[0]); // Zero - Based

    let mut array: [i8; 5] = [1, 2, 3, 4, 5]; //:[type; length]

    println!("{}", array[1]);

    array[0] = 10;

    println!("{}", array[1]);
}
