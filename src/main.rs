#[macro_use]
// imports
extern crate macrosutils;
mod utils;
use macrosutils::say_hello;



//declarations 
#[derive(Debug)]
struct Accountid{
    epoch:i32,
    hash: &'static str
}
impl Accountid{
    pub fn new(epoch:i32,hash: &'static str)-> Accountid{
        Accountid{
            epoch:epoch,
            hash: hash,
        }
    }
}


fn main() {
    let _str = String::from(9.to_string());
    let user1 = Accountid::new(5,"f10xh123546821354353213");
    utils::print(&_str);
    utils::print(&user1);
    self::say_hello!{};
}
