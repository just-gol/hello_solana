use anchor_lang::prelude::*;

declare_id!("2xq6sRfewEU6NAAS9hTbx5Th5e4yDoTiFTjAtdETzZA4");
pub mod instructions;
pub mod states;
use instructions::*;
use states::*;

#[program]
pub mod sol_ji {

    use super::*;
    // 初始化
    pub fn initialize(ctx: Context<InitializeIncense>) -> Result<()> {
        instructions::initialize(ctx)
    }

    // 管理员修改规则
    pub fn update_incense(
        ctx: Context<UpdateIncense>,
        a: IncenseType,
        b: IncenseRule,
    ) -> Result<()> {
        instructions::update_incense(ctx, a, b)
    }

    pub fn nft_mint(ctx: Context<CreateBurnToken>, args: BurnTokenInfoArgs) -> Result<()> {
        instructions::nft_mint(ctx, args)
    }

    // 烧香
    pub fn incense_burn(ctx: Context<CreateIncense>, a: IncenseType) -> Result<()> {
        instructions::incense_burn(ctx, a)
    }

    // 销毁nft
    pub fn destroy(ctx: Context<Destroy>) -> Result<()> {
        instructions::destroy(ctx)
    }

    // 初始化签文
    pub fn initialize_lottery_poetry(ctx: Context<InitializeLotteryPoetry>) -> Result<()> {
        instructions::initialize_lottery_poetry(ctx)
    }

    // 抽签
    pub fn draw_lots(ctx: Context<DrawLots>, count: u8, value: u64) -> Result<()> {
        instructions::draw_lots(ctx, count, value)
    }
}
