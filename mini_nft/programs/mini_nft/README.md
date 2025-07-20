
# Solana Mint NFT 项目说明文档

本项目基于 **Solana + Anchor** 框架，实现一个基础版的 NFT 白名单铸造逻辑。主要功能是：只有在指定时间内，并且在白名单中的用户，才能成功 mint NFT，每个用户只能 mint 一次。

---

## ✨ 一、项目核心业务逻辑

### 1. Admin 设置
- **Admin** 拥有权限初始化系统配置（`Config`），并添加白名单。
- `set_admin` 方法用于创建 Config PDA 账户，保存 admin 的地址。

### 2. 添加白名单
- `add_whitelist(admin, user)` 函数会为每个用户创建一个 PDA 类型的 `Whitelist` 账户，标记：
  - 该用户是否可以 mint（通过 `target` 字段）
  - 有效的 mint 开始/结束时间
  - 是否已经 mint（`is_minted` 字段）

### 3. mint_nft 铸造逻辑
- 核心判断逻辑：
  1. 当前时间是否在允许的 mint 时间区间内（用 `Clock::get()` 获取当前链上时间）
  2. 调用者是否在白名单中（target 匹配）
  3. 是否已 mint（避免重复 mint）
- 成功后调用 `mint_to()` 完成 mint，并更新 `is_minted = true`。

---

## 🔧 二、技术细节说明

### 1. 使用 PDA 实现白名单
- 每个用户的 `Whitelist` 是一个 PDA，seeds 由 `[Whitelist::PREFIX, admin_pubkey, user_pubkey]` 组成。
- `#[account(seeds = [...], bump)]` 会自动校验账户是否匹配 PDA 生成规则。

### 2. 时间控制逻辑
```rust
let now = Clock::get()?.unix_timestamp as u64;
if whitelist.mint_start_time > now || whitelist.mint_end_time < now {
    return Err(error!(ErrorCode::TimeError));
}
```
- `mint_start_time` 和 `mint_end_time` 是链上账户字段，控制 mint 合法期。

### 3. 调试与日志
- 使用 `msg!()` 打印链上日志，方便调试：
```rust
msg!("is_minted: {}, target: {}", ctx.accounts.whitelist.is_minted, ctx.accounts.whitelist.target);
```

### 4. `init_if_needed` vs `init`
- `init_if_needed`：如果账户已存在则不会重新初始化，适用于 `user_ata`, `nft_mint`
- `init`：强制要求账户必须未存在

### 5. 错误处理
使用 `#[error_code]` 定义错误类型，链上可通过日志打印，前端可以捕获 `error.errorCode`。
```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Already minted")]
    AlreadyMinted,
    #[msg("Not in whitelist")]
    NotInWhitelist,
    #[msg("Time out of range")]
    TimeError,
}
```

---

## 🚩 三、测试用例注意事项

### 正常流程
1. 设置 Admin（`set_admin`）
2. Admin 添加某用户为白名单（`add_whitelist(user)`）
3. 用户在有效时间内调用 `mint_nft` 成功

### 异常流程测试（建议使用 try-catch 包裹）
- 非白名单用户尝试 mint（应返回 `NotInWhitelist`）
- 重复 mint（应返回 `AlreadyMinted`）
- 超过时间限制 mint（应返回 `TimeError`）

---

## 📌 四、常见坑点汇总

| 问题描述 | 解决方案 |
|----------|----------|
| `AccountNotInitialized` | PDA 账户未初始化，需确保 `addWhitelist` 已成功执行 |
| `Provided owner is not allowed` | mint 时 `associated_token::authority` 或 `mint::authority` 设置不正确 |
| `unknown signer` | `signers([...])` 中缺少实际参与者的 `Keypair` |
| 测试用例无 log 输出 | `msg!()` 写在错误路径中或未触发，检查测试流程是否命中逻辑 |

---

## 📚 五、涉及 Anchor & Solana 知识点

- Anchor PDA seeds + bump 机制
- `init`, `init_if_needed`, `mut`, `constraint` 注解用法
- SPL Token mint 链上生成逻辑（`anchor_spl::mint_to`）
- Clock 系统变量的使用（链上时间判断）
- 系统账户（SystemAccount）与签名者（Signer）区别
- 如何在 `it()` 测试中捕获错误并验证失败场景

---

## ✅ 六、总结

本项目适合作为一个学习 Solana + Anchor 的综合小项目，涉及了账户权限控制、PDA 应用、mint NFT、白名单判断等核心能力，为后续构建复杂 DApp 打下了基础。

建议延伸改造方向：
- 支持多个 NFT mint
- 支持基于 Metadata 的 NFT（需集成 metaplex）
- 管理端支持批量添加白名单用户
