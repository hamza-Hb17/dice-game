use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub player: Pubkey,
    pub amount: u64,
    pub roll: u8,
    pub slot: u64,
    pub bump: u8,
    pub seed: u128,
}

impl Bet {
    pub fn to_slice(&self) -> Vec<u8> {
        // trasn to slice juste to used in storing data to share it in the sys
        let mut s = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.amount.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&[self.roll, self.bump]);

        s
    }
}
