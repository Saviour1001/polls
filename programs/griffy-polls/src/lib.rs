use anchor_lang::prelude::*;

declare_id!("8fnHv1YLmMFHutXWsus7eZR1GsCYqaC3QKJ3ApLETw7U");

#[program]
pub mod griffy_polls {
    use super::*;

    pub fn initialize_polls_counter(ctx: Context<InitializePollsCounter>) -> Result<()> {
        let polls_counter = &mut ctx.accounts.polls_counter;
        polls_counter.count = 0;
        Ok(())
    }
    

    pub fn create_poll(ctx: Context<CreatePoll>, poll_topic : String, poll_options : Vec<String> ) -> Result<()> {

        let poll_data = &mut ctx.accounts.poll_data;
        
        // assigning all data

        poll_data.poll_topic = poll_topic; 
        poll_data.poll_options = poll_options;
        poll_data.creator = *ctx.accounts.creator.key;
        poll_data.voters = vec![];
        poll_data.votes = [0, 0];
        poll_data.poll_id = ctx.accounts.polls_counter_account.count;

        // incrementing polls_counter
        ctx.accounts.polls_counter_account.count += 1;

        // notifying the event
        emit!(
            PollCreated {
                poll_id: poll_data.poll_id,
                poll_topic: poll_data.poll_topic.clone(),
                poll_options: poll_data.poll_options.clone(),
                creator: poll_data.creator,
            }
        );

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, poll_id: u64, option: u64) -> Result<()> {
        let poll_data = &mut ctx.accounts.poll_data;
        let votes_data = &mut ctx.accounts.votes_data;

        // validating

        if option > 1 {
            return Err(ErrorCode::InvalidOption.into());
        }

        if poll_id != poll_data.poll_id {
            return Err(ErrorCode::InvalidPollId.into());
        }

        if poll_data.voters.contains(&*ctx.accounts.voter.key) {
            return Err(ErrorCode::AlreadyVoted.into());
        }

        // updating data

        poll_data.voters.push(*ctx.accounts.voter.key);
        poll_data.votes[option as usize] += 1;
        votes_data.votes[option as usize] += 1;

        // notifying the event

        emit!(
            PollVoted {
                poll_id: poll_data.poll_id,
                voter: *ctx.accounts.voter.key,
                option: option,
            }
        );


        Ok(())
    }

}


#[derive(Accounts)]
pub struct InitializePollsCounter<'info> {
    #[account(init, payer = creator, space = 8 + 8)]
    pub polls_counter: Account<'info, PollsCounter>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account[mut]]
    pub creator: Signer<'info>,
    #[account(init, payer = creator, space = PollData::size())]
    pub poll_data: Account<'info, PollData>,
    #[account(mut)]
    pub polls_counter_account: Account<'info, PollsCounter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub poll_data: Account<'info, PollData>,
    #[account(mut)]
    pub votes_data: Account<'info, VotesData>,
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
pub struct PollsCounter {
    pub count: u64,
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

#[event]
pub struct PollCreated {
    pub poll_id: u64,
    pub poll_topic: String,
    pub poll_options: Vec<String>,
    pub creator: Pubkey,
}

#[event]
pub struct PollVoted {
    pub poll_id: u64,
    pub voter: Pubkey,
    pub option: u64,
}


