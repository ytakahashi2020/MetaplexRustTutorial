import * as anchor from "@coral-xyz/anchor"; // Anchorの主要ライブラリ
import { Program } from "@coral-xyz/anchor"; // Anchorプログラムの型定義
import { CreateCoreAssetExample } from "../target/types/create_core_asset_example"; // 自動生成されたIDLタイプ
import { Keypair, SystemProgram } from "@solana/web3.js"; // Solana Web3の型とユーティリティ
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core"; // Metaplex CoreプログラムのID

describe("create-core-asset-example", () => {
  // プロバイダの設定
  anchor.setProvider(anchor.AnchorProvider.env());

  // ウォレットとプログラムの設定
  const wallet = anchor.Wallet.local();
  console.log(wallet.publicKey.toString());
  const program = anchor.workspace
    .CreateCoreAssetExample as Program<CreateCoreAssetExample>;

  // アセットのKeypairを生成
  let asset = Keypair.generate();

  // テストケース：アセットの作成
  it("Create Asset", async () => {
    // CreateAssetArgsの設定
    let createAssetArgs = {
      name: "My Asset", // アセット名
      uri: "https://example.com/my-asset.json", // アセットURI
    };

    // トランザクションの送信
    const createAssetTx = await program.methods
      .createCoreAsset(createAssetArgs)
      .accountsPartial({
        asset: asset.publicKey, // 作成するアセットの公開鍵
        // collection: null, // オプションのコレクションアカウント
        authority: null, // オプションの権限アカウント
        payer: wallet.publicKey, // トランザクションの支払者
        owner: null, // 所有者アカウント（オプション）
        updateAuthority: null, // 更新権限アカウント（オプション）
        systemProgram: SystemProgram.programId, // SolanaのシステムプログラムID
        mplCoreProgram: MPL_CORE_PROGRAM_ID, // Metaplex CoreプログラムID
      })
      .signers([asset, wallet.payer]) // 必要な署名者
      .rpc(); // トランザクションを送信

    // トランザクションIDを出力
    console.log("Transaction Signature:", createAssetTx);
  });
});
