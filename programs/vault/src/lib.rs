    use anchor_lang::prelude::*;
    use anchor_lang::system_program::{self, Transfer};

    declare_id!("HU4D7Xis3VBj98AMFWAv1UF3AJ16Xw4FGcuzuRsgH6y7");

    #[program]
    pub mod vault {
        use super::*;

        pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
            let state = &mut ctx.accounts.state;
            state.vault_bump = ctx.bumps.vault;
            state.state_bump = ctx.bumps.state;


            Ok(())
        }

        pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
            let vault = &mut ctx.accounts.vault;
            let state = &mut ctx.accounts.state;

            // Transfer lamports to the vault
            let cpi_accounts = Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: vault.to_account_info(),
            };

            let cpi_program = ctx.accounts.system_program.to_account_info();

            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            system_program::transfer(cpi_ctx, amount)?;

            Ok(())
        }

        pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
            let vault = &mut ctx.accounts.vault;
            let state = &mut ctx.accounts.state;
            let user = &mut ctx.accounts.user;
            let vault_bump = state.vault_bump;
            let state_key = state.key();

            let signer_seeds: &[&[&[u8]]] = &[
                &[
                    b"vault".as_ref(),
                    state_key.as_ref(),
                    &[vault_bump],
                ],
            ];

            // Transfer lamports from the vault
            let cpi_accounts = Transfer {
                from: vault.to_account_info(),
                to: user.to_account_info(),
            };

            let cpi_program = ctx.accounts.system_program.to_account_info();

            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

            system_program::transfer(cpi_ctx, amount)?;

            Ok(())
        }


        pub fn close(ctx: Context<Close>) -> Result<()> {
            let vault = &mut ctx.accounts.vault;
            let state = &mut ctx.accounts.state;
            let user = &mut ctx.accounts.user;
            let vault_bump = state.vault_bump;
            let state_key = state.key();

            let signer_seeds: &[&[&[u8]]] = &[
                &[
                    b"vault".as_ref(),
                    state_key.as_ref(),
                    &[vault_bump],
                ],
            ];

            let balance = vault.get_lamports();
            // get the balance of the vault

            if balance > 0 {
               // Transfer lamports from the vault
            let cpi_accounts = Transfer {
                from: vault.to_account_info(),
                to: user.to_account_info(),
            };

            let cpi_program = ctx.accounts.system_program.to_account_info();

            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

            system_program::transfer(cpi_ctx, balance)?;
            }
            
            Ok(())
        }
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(
            init, 
            payer = user, 
            space = 8 + VaultState::INIT_SPACE, 
            seeds = [b"state".as_ref(), user.key().as_ref()],
            bump)]
        pub state: Account<'info, VaultState>,

        // this is not a regular PDA, but a system account. The vault will only store lamports and not spl
        //Don't need to Init this account, you need to transfer lamports to it and the system program will initialize it
        #[account(
            seeds = [b"vault".as_ref(), state.key().as_ref()],
            bump)]
        pub vault: SystemAccount<'info>, 

        pub system_program: Program<'info, System>,

    }

    /* 
    //this is how it used to be done, example from Builders Cohort Q3 2024s

    impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;
        Ok(())
    }
    }

    */


    #[derive(Accounts)]
    pub struct Deposit<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(
            seeds = [b"state".as_ref(), user.key().as_ref()],
            bump = state.state_bump,
        )]
        pub state: Account<'info, VaultState>,

        #[account(
            mut,
            seeds = [b"vault".as_ref(), state.key().as_ref()],
            bump = state.vault_bump,
        )]
        pub vault: SystemAccount<'info>,

        pub system_program: Program<'info, System>,
    } 

    #[derive(Accounts)]
    pub struct Withdraw<'info> {
        #[account(mut)]
        pub user: Signer<'info>,

        #[account(
            seeds = [b"state".as_ref(), user.key().as_ref()],
            bump = state.state_bump,
        )]
        pub state: Account<'info, VaultState>,

        #[account(
            mut,
            seeds = [b"vault".as_ref(), state.key().as_ref()],
            bump = state.vault_bump,
        )]
        pub vault: SystemAccount<'info>,

        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct Close<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(
            seeds = [b"state".as_ref(), user.key().as_ref()],
            bump = state.state_bump,
        )]
        pub state: Account<'info, VaultState>,

        #[account(
            mut,
            seeds = [b"vault".as_ref(), state.key().as_ref()],
            bump = state.vault_bump,
            //cant close this account with close constraint as it is a SystemAccount and not PDA
        )]
        pub vault: SystemAccount<'info>,

        pub system_program: Program<'info, System>,
    }

    #[account]
    #[derive(InitSpace)]
    pub struct VaultState {
        pub vault_bump: u8,
        pub state_bump: u8,
    }

