

fn main() {
    let x = 200;
    let bin = format!("{x:b}");
    println!("{}", bin);

    let str = 'A';
    let num = str as u8;
    println!("{}", num);

    let bytes = "RuSt".as_bytes();
    for byte in bytes {
        println!("{}", format!("{:b}", byte));
    }
    println!("{:?}", bytes);
}

