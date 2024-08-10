use anchor_lang::prelude::*;
use instructions::*;
use state::game::*;
pub mod error;
pub mod instructions;
pub mod state;

// this key needs to be changed to whatever public key is returned by "anchor keys list
declare_id!("4bSeCUs3fhpfFGGYF4CX2cYpQ89x78D5PA3HDC545ZPj");

#[program]
pub mod tic_tac_toe {
    use instructions::Play;
    use state::Tile;

    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        // Why didn't we just add player_two as an account in the accounts struct? There are two reasons for this.
        //  First, adding it there requires a little more space in the transaction that saves whether the account is writable and whether it's a signer.
        //  But we care about neither the mutability of the account nor whether it's a signer. We just need its address.
        // This brings us to the second and more important reason: Simultaneous network transactions can affect each other if they share the same accounts.
        //  For example, if we add player_two to the accounts struct, during our transaction, no other transaction can edit player_two's account.
        //  Therefore, we block all other transactions that want to edit player_two's account, even though we neither want to read from nor write to the account.
        //  We just care about its address!
        let _ = instructions::setup_game(ctx, player_two);
        Ok(())
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        instructions::play(ctx, tile)
    }
}
