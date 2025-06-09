use soroban_sdk::{contractimpl, Address, Env, Symbol, contracttype, Vec};

pub struct CarbonMarketplace;

#[contracttype]
pub struct Credit {
    pub issuer: Address,
    pub amount: i128,
    pub verified: bool,
    pub buyer: Option<Address>,
}

#[contractimpl]
impl CarbonMarketplace {
    fn credits<'a>(env: &'a Env) -> Vec<'a, Credit> {
        env.storage().instance().get::<Vec<Credit>>(Symbol::short("credits")).unwrap_or(Vec::new(&env))
    }

    pub fn issue_credit(env: Env, amount: i128) {
        let issuer = env.invoker();
        let mut credits = Self::credits(&env);
        credits.push_back(Credit { issuer, amount, verified: false, buyer: None });
        env.storage().instance().set(Symbol::short("credits"), &credits);
    }

    pub fn verify_credit(env: Env, index: u32) {
        let mut credits = Self::credits(&env);
        credits[index as usize].verified = true;
        env.storage().instance().set(Symbol::short("credits"), &credits);
    }

    pub fn buy_credit(env: Env, index: u32) {
        let buyer = env.invoker();
        let mut credits = Self::credits(&env);
        let credit = &mut credits[index as usize];
        assert!(credit.verified, "Not verified");
        credit.buyer = Some(buyer);
        env.storage().instance().set(Symbol::short("credits"), &credits);
    }
}
