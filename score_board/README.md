
# Solana 原生项目示例：Score 增加器

这是一个使用 Solana 原生方式（不使用 Anchor）编写的链上程序项目，包含两个指令：
- `InitScore`：初始化一个数据账户
- `AddScore { amount }`：为分数增加指定数值

---

## 🧱 On-chain 数据结构

```rust
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ScoreAccount {
    pub player: Pubkey,
    pub score: u64,
}
```

---

## 🧠 On-chain 处理逻辑

```rust
pub enum ScoreInstruction {
    InitScore,
    AddScore { amount: u64 },
}

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let data = ScoreInstruction::try_from_slice(instruction_data)?;
        match data {
            ScoreInstruction::InitScore => {
                let accounts_iter = &mut accounts.iter();
                let account = next_account_info(accounts_iter)?;
                let mut score_account = ScoreAccount::try_from_slice(&account.data.borrow())?;
                score_account.player = *account.key;
                score_account.score = 0;
                score_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
            }
            ScoreInstruction::AddScore { amount } => {
                let accounts_iter = &mut accounts.iter();
                let account = next_account_info(accounts_iter)?;
                let mut score_account = ScoreAccount::try_from_slice(&account.data.borrow())?;
                if *account.key == score_account.player {
                    score_account.score += amount;
                    score_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
                }
            }
        }
        Ok(())
    }
}
```

---

## 🧪 客户端调用逻辑

```rust
fn init(...) {
    let create_tx = system_instruction::create_account(...);
    let instruction = borsh::to_vec(&ScoreInstruction::InitScore).unwrap();
    let init_ix = Instruction {
        program_id: *program_id,
        accounts: vec![AccountMeta::new(score_account.pubkey(), false)],
        data: instruction,
    };

    let tx = Transaction::new_signed_with_payer(
        &[create_tx, init_ix],
        Some(&payer.pubkey()),
        &[&payer, &score_account], // ✅ 账户签名
        recent_blockhash,
    );
}
```

```rust
fn add(...) {
    let instruction = borsh::to_vec(&ScoreInstruction::AddScore { amount: 1 }).unwrap();
    let add_ix = Instruction {
        program_id: *program_id,
        accounts: vec![AccountMeta::new(score_account.pubkey(), false)],
        data: instruction,
    };

    let tx = Transaction::new_signed_with_payer(
        &[add_ix],
        Some(&payer.pubkey()),
        &[payer], // ✅ 只需要 payer 签名
        recent_blockhash,
    );
}
```

---

## ⚠️ 注意事项

- 初始化账户时（`InitScore`）：
  - 需要使用 `Keypair::new()` 创建账户地址。
  - 需要使用 `system_instruction::create_account()` 创建数据账户。
  - **签名者：** 需要 `payer` 和 `score_account` 都签名。

- 增加分数时（`AddScore`）：
  - 不需要重新创建账户。
  - 直接使用上一次创建的 `score_account` 即可。
  - **签名者：** 只需要 `payer` 签名即可。

---

## ✅ 使用方式建议

- 使用 `borsh` 做序列化反序列化，务必导入 `BorshSerialize` 和 `BorshDeserialize`。
- 用 `solana account <pubkey>` 可查看链上账户的变化。
- 建议每个指令对应一个函数，保持客户端调用逻辑清晰。

