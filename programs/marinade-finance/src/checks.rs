use crate::CommonError;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake::state::StakeState;
use anchor_spl::token::{Mint, TokenAccount};

pub fn check_min_amount(amount: u64, min_amount: u64, action_name: &str) -> Result<()> {
    if amount >= min_amount {
        Ok(())
    } else {
        msg!(
            "{}: Number too low {} (min is {})",
            action_name,
            amount,
            min_amount,
        );
        Err(CommonError::NumberTooLow.into())
    }
}

pub fn check_address(
    actual_address: &Pubkey,
    reference_address: &Pubkey,
    field_name: &str,
) -> Result<()> {
    if actual_address == reference_address {
        Ok(())
    } else {
        msg!(
            "Invalid {} address: expected {} got {}",
            field_name,
            reference_address,
            actual_address
        );
        Err(ProgramError::InvalidArgument.into())
    }
}

pub fn check_owner_program<'info, A: ToAccountInfo<'info>>(
    account: &A,
    owner: &Pubkey,
    field_name: &str,
) -> Result<()> {
    let actual_owner = account.to_account_info().owner;
    if actual_owner == owner {
        Ok(())
    } else {
        msg!(
            "Invalid {} owner_program: expected {} got {}",
            field_name,
            owner,
            actual_owner
        );
        Err(ProgramError::InvalidArgument.into())
    }
}

pub fn check_mint_authority(
    mint: &Mint,
    mint_authority: Pubkey,
    field_name: &str,
) -> Result<()> {
    if mint.mint_authority.contains(&mint_authority) {
        Ok(())
    } else {
        msg!(
            "Invalid {} mint authority {}. Expected {}",
            field_name,
            mint.mint_authority.unwrap_or_default(),
            mint_authority
        );
        Err(ProgramError::InvalidAccountData.into())
    }
}

pub fn check_freeze_authority(mint: &Mint, field_name: &str) -> Result<()> {
    if mint.freeze_authority.is_none() {
        Ok(())
    } else {
        msg!("Mint {} must have freeze authority not set", field_name);
        Err(ProgramError::InvalidAccountData.into())
    }
}

pub fn check_mint_empty(mint: &Mint, field_name: &str) -> Result<()> {
    if mint.supply == 0 {
        Ok(())
    } else {
        msg!("Non empty mint {} supply: {}", field_name, mint.supply);
        Err(ProgramError::InvalidArgument.into())
    }
}

pub fn check_token_mint(token: &TokenAccount, mint: Pubkey, field_name: &str) -> Result<()> {
    if token.mint == mint {
        Ok(())
    } else {
        msg!(
            "Invalid token {} mint {}. Expected {}",
            field_name,
            token.mint,
            mint
        );
        Err(ProgramError::InvalidAccountData.into())
    }
}

pub fn check_token_owner(token: &TokenAccount, owner: &Pubkey, field_name: &str) -> Result<()> {
    if token.owner == *owner {
        Ok(())
    } else {
        msg!(
            "Invalid token account {} owner {}. Expected {}",
            field_name,
            token.owner,
            owner
        );
        Err(ProgramError::InvalidAccountData.into())
    }
}

// check that the account is delegated and to the right validator
// also that the stake amount is updated
pub fn check_stake_amount_and_validator(
    stake_state: &StakeState,
    expected_stake_amount: u64,
    validator_vote_pubkey: &Pubkey,
) -> Result<()> {
    let currently_staked = if let Some(delegation) = stake_state.delegation() {
        if delegation.voter_pubkey != *validator_vote_pubkey {
            msg!(
                "Invalid stake validator index. Need to point into validator {}",
                validator_vote_pubkey
            );
            return Err(ProgramError::InvalidInstructionData.into());
        }
        delegation.stake
    } else {
        return Err(CommonError::StakeNotDelegated.into());
    };
    // do not allow to operate on an account where last_update_delegated_lamports != currently_staked
    if currently_staked != expected_stake_amount {
        msg!(
            "Operation on a stake account not yet updated. expected stake:{}, current:{}",
            expected_stake_amount,
            currently_staked
        );
        return Err(CommonError::StakeAccountNotUpdatedYet.into());
    }
    Ok(())
}
