### 1 create an anchor project

`anchor init <projectName>`

### 2 set rust-analyzer

.vscode/settings.json

```
{
  "rust-analyzer.linkedProjects": [
    "<cargo.toml path>"
  ]
}
```

### 3 set to devnet

- Anchor.toml

- faucet
  `https://faucet.solana.com/`

### 4 import anchor_lang

`use anchor_lang::prelude::*;`

### 5 set program id

`declare_id!("")`

`anchor keys sync`

### 6 create args

#### 1 create a struct

```

pub struct CreateAssetArgs {
name: String,
uri: String,
}

```

#### 2 set derive macro

`#[derive(AnchorDeserialize, AnchorSerialize)]`

### 7 create accounts

#### 1 create an outline

```

#[derive(Accounts)]
pub struct CreateAsset<'info> {}

```

#### 2 create fields

```

pub asset,
pub payer,
pub system_program,
pub mpl_core_program

```

#### 3 set the type

`https://github.com/coral-xyz/anchor/blob/v0.30.1/lang/src/accounts/mod.rs`

```

/// CHECK: this account is checked by the address constraint

```

#### 4 set account validation

`https://www.anchor-lang.com/docs/account-constraints`

### 8 create a program

```

#[program]
pub mod create_core_asset_example {
use super::\*;
}

```

### 9 create a function

#### 1 create an outline

````

pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
0k(())
} ```

````

#### 2

CreateV2CpiBuilder::new
`mpl_core::instructions::CreateV2CpiBuilder`

```
- asset
- payer
- system_program
- name
- uri
- invoke
```

use to_account_info()

### 10 deplooy

`anchor build`

`anchor deploy`
