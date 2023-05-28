// token.rs
// public definition of the interface

// this indicates that Token structure should be in it's own account
#[account];
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
// automatically derives an account address based on the type
// if the Token type is in Account A, MyTokenData should be in A.pubkey.with_seed("MyTokenData")
#[associated_account(Token)];
struct MyTokenData {
    counter: u64,
}

// make sure this is a public symbol that gets resolved at link time
#[interface]
impl TokenInterface for Token {
    fn transfer(from: &mut Token, to: &mut Token, amount: u64) {
        let my_data: &mut MyTokenData  = get_associated_account<MyTokenData>(from);

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
