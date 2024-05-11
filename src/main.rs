fn arrays_and_loops() {
    let mut arr = [1, 2, 3, 4, 5];
    square(&mut arr[2]);
    println!("{:?}", arr);

    // for loop with index only
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    for i in 0..=arr.len() {
        println!("{}", arr[i]);
    }

    // for loop with value only
    for v in arr.iter() {
        println!("{}", v);
    }

    // for loop with index and value
    for (i, v) in arr.iter().enumerate() {
        println!("{}: {}", i, v);
    }

    // for loop with mutable reference to each element
    for i in arr.iter_mut() {
        square(i);
    }
    println!("{:?}", arr);

    // for loop with mutable reference to the entire array
    for i in &mut arr {
        square(i);
    }
    println!("{:?}", arr);

    // for loop with immutable reference to the entire array (aka a slice?)
    for i in &arr {
        println!("{}", i);
    }
}

fn square(i: &mut i32) {
    *i *= *i;
}

fn ownership_and_borrowing() {
    // "move", i.e. transfer ownership
    let s1 = String::from("hello world");
    let s2 = s1; // this can also be a function call that takes ownership of s1, e.g. func1(s1)
                 // println!("{}", s1); // error: value borrowed here after move
    println!("{}", s2);

    // one solution is to clone the value
    let s1 = String::from("hello world");
    let s2 = s1.clone();
    println!("{}", s1);

    // a better solution is to "borrow" the value using a reference
    let s1 = String::from("hello world");
    let s2 = &s1;
    println!("{}", s1);
    println!("{}", s2);

    // cannot mix mutable and immutable borrows
    let mut s1 = String::from("hello world");
    let s2 = &s1; // immutable borrow occurs here
                  // s1.push_str("1"); // mutable borrow occurs here
    println!("{}", s1);
    println!("{}", s2);
    // solution: either mutate first then borrow
    let mut s1 = String::from("hello world");
    s1.push_str("1");
    let s2 = &s1;
    println!("{}", s1);
    println!("{}", s2);
    // or finish using the immutable borrow before mutating
    let mut s1 = String::from("hello world");
    let s2 = &s1;
    println!("{}", s2);
    s1.push_str("1");
    println!("{}", s1);
}

// optionals
fn lookup_player(id: u32) -> Option<String> {
    if id == 1 {
        return Some("crabby".to_string());
    }
    return None;
}

fn run_game() -> Option<()> {
    // let player = match lookup_player(1) {
    //     Some(p) => p,
    //     None => return,
    // };
    let player = lookup_player(1)?;
    println!("Player {}", player);
    Some(())
}

fn main() {
    run_game();
}
