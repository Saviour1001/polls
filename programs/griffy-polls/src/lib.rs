use anchor_lang::prelude::*;

declare_id!("8fnHv1YLmMFHutXWsus7eZR1GsCYqaC3QKJ3ApLETw7U");

#[program]
pub mod griffy_polls {
    use super::*;

    pub fn create_poll(ctx: Context<CreatePoll>, poll_topic : String, poll_options : Vec<String> ) -> Result<()> {

        let poll_data = &mut ctx.accounts.poll_data;
        // poll_data.poll_id = ctx.accounts.polls_len.len;
        poll_data.poll_topic = poll_topic;
        poll_data.poll_options = poll_options;
        poll_data.creator = *ctx.accounts.creator.key;
        poll_data.voters = vec![];
        poll_data.votes = [0, 0];
        // ctx.accounts.polls_len.len += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account[mut]]
    pub creator: Signer<'info>,
    #[account(init, payer = creator, space = PollData::size())]
    pub poll_data: Account<'info, PollData>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PollData {
    pub poll_id: u64,
    pub poll_topic: String,
    pub poll_options: Vec<String>,
    pub creator: Pubkey,
    pub voters: Vec<Pubkey>,
    pub votes: [u64; 2],
}

#[account]
pub struct PollsLen {
    pub len: u64,
}

#[account]
pub struct VotesData {
    pub votes: [u64; 2],
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const U64_LENGTH: usize = 8;
const VECTOR_LENGTH: usize = 4;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 280 * 4; // 280 chars max.
const MAX_OPTION_LENGTH: usize = 25 * 4; // 25 chars max.
const UPPER_BOUND_FOR_VOTERS: u64 = 100;

impl PollData {
    pub fn size() -> usize {
        DISCRIMINATOR_LENGTH + // anchor account discriminator
        PUBLIC_KEY_LENGTH + // creator
        STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH +  // poll topic length
        VECTOR_LENGTH + (MAX_OPTION_LENGTH * 2)+ // poll options length, takes 2 options
        VECTOR_LENGTH + (PUBLIC_KEY_LENGTH * UPPER_BOUND_FOR_VOTERS as usize) + // voters
        U64_LENGTH * 2 // votes
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid option. Must be 0 or 1.")]
    InvalidOption,
    #[msg("You have already voted in this poll.")]
    AlreadyVoted,
    #[msg("Invalid poll ID.")]
    InvalidPollId,
}
