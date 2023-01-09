fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.push(6);
    nums.pop();
    println!("{:?}", nums);

    let mut vector = Vec::new();
    vector.push("String");
    println!("{:?}", vector);

    vector.reverse();

    let mut testVector = Vec::<i32>::with_capacity(2);
    println!("{}", testVector.capacity());

    let v: Vec<i32> = (0..10).collect();
    println!("{:?}", v);
}
