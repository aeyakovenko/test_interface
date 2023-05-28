// token.rs
// public definition of the interface
pub struct Token {
    authority: Pubkey,
    balance: u64
}

//default method implementations for convinience
impl Token {
    fn transfer(&mut self, to: &mut Token, amount: u64) -> Result<()> {
        let account = get_account_from_object(self);
        account.check_is_signed()?;
        self.balance.checked_sub(amount)?;
        to.balance.checked_add(amount)
    }
}

//this is the method that is expected to be exported by programs
pub trait TokenInterface {
    fn transfer(from: &mut Token, to: &mut Token, amount: u64) -> Result<()>
}
// end token.rs

// myprogram.rs
// private implementation of the token interface
struct MyTokenData {
    counter: u64,
}

// my token initializer
// also initializes an associated object that is derived from the account address
pub fn init_my_token(account: &mut Account) -> Result<()> {
    account.init(Token::default())?;
    //get the associated account
    let mut associated_account = get_account(account.pubkey.with_seed("mydata"))?;
    associated_account.init(MyTokenData::default());
}

//we need some way to make sure that 
//these symbols resolve to the same symbol on all programs
#[interface]
impl TokenInterface for Token {
    fn transfer(from: &mut Token, to: &mut Token, amount: u64) {
        let account = get_account_from_object(from);

        //get the associated object
        let mut my_data_account = get_account_from_pubkey(account.pubkey.with_seed("mydata"));
        let mut my_data: MyTokenData = my_data_account.get_object();

        //my custom code
        my_data.counter += 1;

        //call the default implemtation
        from.transfer(to, amount)
    }
}
// end myprogram.rs

//swap.rs
//token swapper implementation 

fn swap(src_a: &mut Token, src_b: &mut Token, dst_a: &mut Token, dst_b: &mut Token, size_a: u64, size_b: u64) {
    //src_a is owned by ProgramA, which resolves to the `impl TokenInterface for Token in ProgramA'
    src_a.transfer(dst_a, size_a);
    //src_b is owned by ProgramB, which resolves to the `impl TokenInterface for Token in ProgramB'
    src_b.transfer(dst_b, size_b);
}
// end swap.rs
