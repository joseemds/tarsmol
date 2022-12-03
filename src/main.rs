// From https://www.gnu.org/software/tar/manual/html_node/Standard.html
struct Header {
    data: [u8; 512],
};


struct StarHeader{
    name: [u8, 100],
    mode: [u8, 8],
    uid: [u8, 8],
    gid: [u8, 8],
    size: [u8,12],
    msize: [u8,12],
    chksum: [u8, 8],
    typeflag: u8,
    linkname: [u8, 100],
    magic: [u8, 6],
    version: [u8, 2],
    uname: [u8, 32],
    gname: [u8, 32],
    devmajor: [u8, 8],
    devminor: [u8, 8],
    prefix: [u8, 155],

};


struct Archive {
    header: [u8; 512],

}
fn main() {
    let x = Archive {
        header: Default::default(),

    };
    println!("Hello, world!");
}
