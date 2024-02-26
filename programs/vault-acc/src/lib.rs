use anchor_lang::prelude::*;

declare_id!("9pU1ESxdgeWPKjQwg6R7vuVom8rno7L8CNb76EUh6vVN");

#[program]
pub mod vault_acc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
