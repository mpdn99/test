use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
#[derive(Default)]
pub struct Merchant {
    pub merchant_wallet_address: Pubkey,
    pub name: String,
    pub description: String,
    pub logo_url: String,
    pub product_count: u64,
    pub current_customer_key: Pubkey
}

impl Merchant {
    pub const MERCHANT_PREFIX: &'static str = "merchant";
    pub const NAME_MAX_LEN: usize = 128;
    pub const DESCRIPTION_MAX_LEX: usize = 256;
    pub const LOGO_URL_MAX_LEN: usize = 256;

    pub fn space() -> usize {
        8 + //discriminator
        1 + //bump
        4 + //name
        4 + //description
        4 + //logo
        8 + //product_count
        4 + std::mem::size_of::<Pubkey>()
    }
}