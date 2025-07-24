use anchor_lang::{
    prelude::*,
    solana_program::lamports,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault",house.key().as_ref()], // Setting house.key as the unique key for each house's vault (vault unique for each house)
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>, // we need system_program to do the transfer
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, amount: u64) -> Result<()> {
        let ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.house.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );

        transfer(ctx, amount)
    }
}
