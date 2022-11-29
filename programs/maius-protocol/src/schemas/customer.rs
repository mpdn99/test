use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(Default)]
pub struct Customer {
    pub authority: Pubkey,
    pub prev_customer_key: Pubkey,
    pub description: String,
    pub created: i64,
    pub subscription_count: u8,
}

impl Customer {
    pub const CUSTOMER_PREFIX: &'static str = "customer";
    pub const DESCRIPTION_MAX_LEN: usize = 512;

    pub fn space() -> usize {
        8 + //discriminator
        1 + //bump
        PUBKEY_SIZE + //authority
        PUBKEY_SIZE + //pre_customer_key
        4 + //Vec len
        Customer::DESCRIPTION_MAX_LEN + //description
        8 +
        I64Size
    }
}