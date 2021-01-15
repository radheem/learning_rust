#[macro_use]
// imports
// mod macrosutils;
mod utils;



//declarations 
#[derive(Debug)]
pub struct UserAccount<T,G>{
    pk: T,
    epoch:G,
    hash: &'static str,
    amount: i32
}
trait Account{
    fn new<T,G>(pk: T,epoch:G,hash: &'static str)-> Self;
    fn transfer<T>(x:&mut T,y:&mut T,amount: i32);
    fn credit<T>(acc: &mut T,amount:i32);
    fn debit<T>(acc: &mut T,amount:i32);
}
impl Account for UserAccount{
    fn new<T,G>(pk: T, epoch:G, hash: &'static str)-> UserAccount{
        UserAccount{
            pk: pk,
            epoch:epoch,
            hash: hash,
            amount: 0
        }
    }
    fn transfer<UserAccount>(x:&mut UserAccount,y: &mut UserAccount,amount: i32){
        assert!(x.amount > amount,"the sender doesn't have enough funds");
        x.amount -= amount;
        y.amount += amount;
    }
    fn credit<UserAccount>(acc: &mut UserAccount,amount:i32){
        acc.amount += amount;
    }
    fn debit<UserAccount>(acc: &mut UserAccount,amount:i32){
        assert!(acc.amount >= amount,"the sender doesn't have enough funds");
        acc.amount -= amount;
    }
}


fn main() {
    let user1 = UserAccount::new::<i64,i32>(146186156816261536,5,"f10xh123546821354353213");
    utils::print(&user1);
}
