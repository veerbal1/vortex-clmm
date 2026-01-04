use anchor_lang::prelude::*;

declare_id!("69kFA3bXWxaGLxz9iiMenWpE32t1hoJb8WzdvVbTzLN2");

#[program]
pub mod vortex_clmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
