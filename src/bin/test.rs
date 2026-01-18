use opal_kit::Disk;
use std::fs::File;

fn main() {
    let f = Disk::open("/dev/nvme0").unwrap();

    println!("{:?}", f.get_id());
}
