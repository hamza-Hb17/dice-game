use anchor_lang::prelude::*;

declare_id!("8jxX1Jig2hBdhVLfiDcWNPfqjvJAF7Gpq23sUB3ApTow");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
