use anchor_lang::prelude::*;

declare_id!("23cgJC6z7oB1mp1Rcbh9SncsKDmUUdvZCe1kxfQvpSn1");

#[program]
pub mod test_upgrade {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
