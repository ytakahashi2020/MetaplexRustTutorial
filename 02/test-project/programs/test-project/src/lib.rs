use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    // accounts::BaseCollectionV1,
    instructions::CreateV2CpiBuilder,
};

declare_id!("7YruNdwaLC2jvNrzwkQzRW5XLprG4jQ7Z687qV1YbK2Y");

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    name: String, // アセットの名前
    uri: String,  // アセットのURI
}

#[program]
pub mod create_core_asset_example {
    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        // Optionalアカウントを処理
        // let collection = match &ctx.accounts.collection {
        //     Some(collection) => Some(collection.to_account_info()),
        //     None => None,
        // };

        let authority = match &ctx.accounts.authority {
            Some(authority) => Some(authority.to_account_info()),
            None => None,
        };

        let owner = match &ctx.accounts.owner {
            Some(owner) => Some(owner.to_account_info()),
            None => None,
        };

        let update_authority = match &ctx.accounts.update_authority {
            Some(update_authority) => Some(update_authority.to_account_info()),
            None => None,
        };

        // CreateV2CpiBuilderを使ってCPI命令を作成・実行
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            // .collection(collection.as_ref())
            .authority(authority.as_ref())
            .payer(&ctx.accounts.payer.to_account_info())
            .owner(owner.as_ref())
            .update_authority(update_authority.as_ref())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?; // CPI命令を実行

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub asset: Signer<'info>, // アセットの署名者（書き込み可能）

    #[account(mut)]
    // pub collection: Option<Account<'info, BaseCollectionV1>>, // コレクションアカウント（オプション）

    pub authority: Option<Signer<'info>>, // 権限アカウント（オプション）

    #[account(mut)]
    pub payer: Signer<'info>, // 手数料を支払うアカウント

    /// CHECK: このアカウントはmpl_coreプログラムで検証されます
    pub owner: Option<UncheckedAccount<'info>>,

    /// CHECK: このアカウントはmpl_coreプログラムで検証されます
    pub update_authority: Option<UncheckedAccount<'info>>,

    pub system_program: Program<'info, System>, // Solanaのシステムプログラム

    #[account(address = MPL_CORE_ID)]
    /// CHECK: このアカウントはアドレス制約で検証されます
    pub mpl_core_program: UncheckedAccount<'info>, // Metaplex Coreプログラム
}
