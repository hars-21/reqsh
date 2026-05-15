use crate::{help::get_help, parser::Command, request::Request, runner::fetch, state::State};

pub fn execute_command(cmd: Command, ctx: &mut State) -> Result<(), String> {
    match cmd {
        Command::Base(url) => {
            ctx.set_base_url(&url);
            Ok(())
        }
        Command::Header(k, v) => {
            ctx.set_header(k, v);
            Ok(())
        }
        Command::Help => {
            let help = get_help();
            println!("{}", help);
            Ok(())
        }
    }
}

pub fn execute_request(req: Request, ctx: &mut State) -> Result<String, String> {
    let base_url = ctx.get_base_url();

    fetch(&req, base_url)
}
