extern crate gcc;

fn main() {
    gcc::Config::new().file("block_utils.c").flag("-fblocks").compile("libblock_utils.a");
}
