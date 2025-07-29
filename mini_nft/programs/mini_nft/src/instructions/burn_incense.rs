use anchor_lang::prelude::*;
pub fn create_burn_incense(ctx: Context<CreateBurnIncense>, flavor: Flavor) -> Result<()> {
    let rule = Flavor::rule(&flavor);
    rule.incense_value;
    rule.merit_value;
    rule.payment;
    rule.incense_value;
    Ok(())
}

pub enum Flavor {
    FaintScent,
    StrongScent,
    DivineScent,
}
#[account]
pub struct FlavorRule {
    pub payment: f32,
    pub merit_value: u32,
    pub incense_value: u32,
}

impl Flavor {
    pub fn rule(&self) -> FlavorRule {
        match self {
            Flavor::FaintScent => FlavorRule {
                payment: 0.1,
                merit_value: 10,
                incense_value: 5,
            },
            Flavor::StrongScent => FlavorRule {
                payment: 0.1,
                merit_value: 10,
                incense_value: 5,
            },
            Flavor::DivineScent => FlavorRule {
                payment: 0.1,
                merit_value: 10,
                incense_value: 5,
            },
        }
    }
}

#[derive(Accounts)]
pub struct CreateBurnIncense<'info> {
    pub authority: Signer<'info>,
}
