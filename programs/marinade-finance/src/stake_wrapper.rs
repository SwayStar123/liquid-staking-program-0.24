use std::ops::Deref;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake::state::StakeState;

#[account]
pub struct StakeWrapper {
    pub inner: StakeState,
}

impl Deref for StakeWrapper {
    type Target = StakeState;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
