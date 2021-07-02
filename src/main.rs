fn main() {
    let welcome: &str = "welcome";
    let mil = welcome;
    println!("el valor de {}",mil);
    let list: [i32; 3] = [1, 2, 3];
    println!("el segundo numero es {}",list[1]);
    println!("la cantidad de numeros son {}",list.len());

    let f: fn(i32) -> i32 = plus_one;
}

fn plus_one(i: i32) -> i32 {
    i + 1
}

