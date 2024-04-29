use shrs::prelude::{styled_buf::StyledBuf, *};
use shrs_command_timer::CommandTimerState;

pub struct SimplePrompt;

impl Prompt for SimplePrompt {
    fn prompt_left(&self, state: &LineStateBundle) -> StyledBuf {
        let indicator = match state.line.mode() {
            LineMode::Insert => String::from(">").cyan(),
            LineMode::Normal => String::from(":").yellow(),
        };
        if !state.line.lines.is_empty() {
            return styled_buf! {" ", indicator, " "};
        }

        styled_buf!(
            " ",
            username().map(|u| u.blue()),
            " ",
            top_pwd().white().bold(),
            " ",
            indicator,
            " "
        )
    }
    fn prompt_right(&self, line_ctx: &LineStateBundle) -> StyledBuf {
        let time_str = line_ctx
            .ctx
            .state
            .get::<CommandTimerState>()
            .and_then(|x| x.command_time())
            .map(|x| format!("{x:?}"));

        if !line_ctx.line.lines.is_empty() {
            return styled_buf!("");
        }
        /*
        if let Ok(git_branch) = git::branch().map(|s| format!("git:{s}").blue().bold()) {
            styled_buf!(git_branch, " ", time_str, " ")
        } else {
            styled_buf!(time_str, " ")
        }
        */
        styled_buf!(time_str, " ")
    }
}

// A prompt that shows only the bare minimum details
pub struct MinimalPrompt;

impl Prompt for MinimalPrompt {
    fn prompt_left(&self, line_ctx: &LineStateBundle) -> StyledBuf {
        todo!()
    }

    fn prompt_right(&self, line_ctx: &LineStateBundle) -> StyledBuf {
        todo!()
    }
}

// Powerline inspired prompt
pub struct PowerlinePrompt;

impl Prompt for PowerlinePrompt {
    fn prompt_left(&self, line_ctx: &LineStateBundle) -> StyledBuf {
        todo!()
    }

    fn prompt_right(&self, line_ctx: &LineStateBundle) -> StyledBuf {
        todo!()
    }
}
