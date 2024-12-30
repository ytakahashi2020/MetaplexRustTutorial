use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    instructions::CreateV2CpiBuilder,
};

declare_id!("7YruNdwaLC2jvNrzwkQzRW5XLprG4jQ7Z687qV1YbK2Y");

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    name: String, 
    uri: String, 
}

#[program]
pub mod create_core_asset_example {
    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {

        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .payer(&ctx.accounts.payer.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?; 

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub asset: Signer<'info>, 

    #[account(mut)]
    pub payer: Signer<'info>, 

    pub system_program: Program<'info, System>, 

    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>, 
}
