use std::fs;
use std::str;

// From https://www.gnu.org/software/tar/manual/html_node/Standard.html
struct StarHeader {
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    msize: [u8; 12],
    chksum: [u8; 8],
    typeflag: u8,
    linkname: [u8; 100],
    magic: [u8; 6],
    version: [u8; 2],
    uname: [u8; 32],
    gname: [u8; 32],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    prefix: [u8; 155],
}

struct Archive {
    header: [u8; 512],
    blocks: Vec<[u8; 512]>,
}

struct Entry {
    block: [u8; 512]
}


fn octal_to_decimal(num: &str) -> i32 {
     i32::from_str_radix(num, 8).unwrap()
}

fn next_block(file_size: &str) -> i32 {
    let file_size = octal_to_decimal(file_size);
    if file_size % 512 != 0 {
        (file_size / 512) + 2
    } else {
        (file_size / 512) + 1
    }
}

fn main() {
    let tarball = fs::read("smol.tar").unwrap();
    let entries = tarball.chunks(512).collect::<Vec<_>>();
    println!("{:?}", entries);
    let first_file = entries[0];
    let first_file_size = std::str::from_utf8(&first_file[124..135]).unwrap().replace("\0", "");
    println!("First File name: {}", std::str::from_utf8(&first_file[0..99]).unwrap().replace("\0", ""));
    let next_file = next_block(first_file_size.as_str());
    let second_file = entries[next_file as usize];
    println!("Second File name: {}", std::str::from_utf8(&second_file[0..99]).unwrap().replace("\0", ""));

    println!("Hello, world!");
}
