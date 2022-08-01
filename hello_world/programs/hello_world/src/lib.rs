use anchor_lang::prelude::*;
declare_id!("BoEsGZys5GVBD7UGQugH4KX7iD1oHafGNmiQin7jni3N");

#[program]
pub mod hello_world {
    use super::*;

    pub fn hello(_ctx: Context<Initialize>) -> Result<()> {
        msg!("hello world!!!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
