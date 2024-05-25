use std::io::Read;

fn for_chunk_type() {
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

fn for_chunk() {
    let data_length: u32 = 42;
    let chunk_type: &[u8] = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let chunk_data: Vec<u8> = data_length
        .to_be_bytes()
        .iter()
        .chain(chunk_type.iter())
        .chain(message_bytes.iter())
        .chain(crc.to_be_bytes().iter())
        .copied()
        .collect();

    // try_from()
    let mut value: &[u8] = chunk_data.as_ref();

    let mut length_data: [u8; 4] = [0; 4];
    let mut chunk_type_data: [u8; 4] = [0; 4];
    let mut crc_data: [u8; 4] = [0; 4];
    let mut data: Vec<u8> = Vec::new();

    value.read_exact(&mut length_data).unwrap();
    value.read_exact(&mut chunk_type_data).unwrap();
    let length = u32::from_be_bytes(length_data);

    let mut iter = value.iter();
    let mut index = 0;
    while index < length {
        data.push(iter.next().unwrap().clone());
        index += 1;
    }

    let mut new_value: &[u8] = iter.as_ref();

    new_value.read_exact(&mut crc_data).unwrap();
    let crc2 = u32::from_be_bytes(crc_data);

    println!("{}", length);
    println!("{}", String::from_utf8(chunk_type_data.into()).unwrap());
    println!("{}", String::from_utf8(data).unwrap());
    println!("{}", crc2);
}

fn main() {
    for_chunk();
}
