
# Solana 内容平台合约设计文档

## 🧾 项目需求

本项目旨在构建一个 **内容创作者平台**，支持以下核心功能：

- 创作者可发布文章。
- 用户可通过平台代币打赏作者。
- 平台以 PDA 形式收取部分费用并支持提现。
- 使用 Anchor 框架和 SPL Token 实现安全的 Token 流转和账户管理。

---

## ✅ 已实现功能

### 1. **创建 Mint 和初始发币**
- 使用 `create_mint` 初始化一个 mint（2位小数）和一个用户的 ATA。
- 通过 PDA `treasury` 作为 mint authority，调用 `mint_to` 进行代币分发。

### 2. **打赏作者**
- 用户通过 `reward_author` 从自己的 ATA 转账代币至平台 `treasury_ata`。
- 校验 `giver_ata` 的 owner 为调用者，避免伪造转账。
- `treasury_ata` 是由 PDA `treasury` 控制的 token account。

### 3. **平台提现**
- 通过 `withdraw` 方法将平台账户中的代币转出至指定接收者的 ATA。
- 使用 PDA `treasury` 控制 authority，确保只有合约逻辑可以操作代币。
- 支持自动初始化接收方 ATA。

### 4. **文章发布与作者档案**
- 创作者通过 `create_article` 发布文章，同时创建或更新自己的 `AuthorProfile`。
- 文章以 `article` PDA 账户存储，含作者、公钥、时间戳等。
- 作者档案中维护所有发表过的文章公钥列表。

### 5. **分页查看文章**
- `view_articles` 支持分页查看指定作者的文章列表，避免一次性返回过多数据。

---

## ⚠️ 开发重点与注意事项

### ✅ PDA 签名
- `mint_to` 与 `withdraw` 中均使用 `CpiContext::new_with_signer`，为 PDA（如 `treasury`）提供 seeds 进行签名模拟。
- 如果使用 PDA 作为 token authority，则必须使用 `new_with_signer`，否则报错：`Signature verification failed`。

### ✅ ATA 所属者校验
- 在 `reward_author` 中使用 `constraint = giver_ata.owner == giver_ata_wallet.key()` 保证账户归属合法，防止他人代打赏或绕过权限。

### ✅ token::transfer 的安全性
- 转账时需保证 `authority` 是账户 owner，且是 signer。
- 如果 owner 是 PDA，需提供 signer seeds；否则使用用户钱包签名。

### ✅ Article 与 AuthorProfile 的初始化
- Article 的 PDA 种子为 `[b"article", author.key(), title]`。
- AuthorProfile 的 PDA 种子为 `[b"author_profile", author.key()]`，每个作者唯一。

### ✅ 使用 `UncheckedAccount` 的场景
- 在 `create_mint` 中，`giver_ata_wallet` 只用作 `associated_token::authority`，无需签名，可安全使用 `UncheckedAccount<'info>`。

---

## 📦 依赖说明

- Anchor Framework
- SPL Token / Associated Token Program
- 系统程序 SystemProgram

---

## 📁 文件结构建议（可选）

```
src/
  ├─ lib.rs
  ├─ instructions/
  │    ├─ create_mint.rs
  │    ├─ reward_author.rs
  │    ├─ withdraw.rs
  │    ├─ create_article.rs
  │    └─ view_articles.rs
  └─ state/
       ├─ article.rs
       └─ author.rs
```

---

## ✅ 后续建议

- 为 `Article` 增加内容哈希以防篡改。
- 引入代币打赏比例分成机制。
- 加入时间戳和权限管理（如管理员收款）。
- 使用 `event!` 宏发送链上事件，便于前端监听。
