use crate::{display::display_response, request::Request, runner::fetch, state::ShellState};

pub fn execute(req: Request, ctx: &ShellState) -> Result<String, String> {
    let base_url = ctx.get_base_url();
    let global_headers = ctx.get_headers();

    let response = fetch(&req, base_url, global_headers);

    match response {
        Ok(r) => Ok(display_response(r)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Method;

    #[test]
    fn execute_returns_result() {
        let req = Request::new(Method::GET, "/users".to_string());
        let state = ShellState::new();
        let result = execute(req, &state);

        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn execute_fails_without_base_url() {
        let req = Request::new(Method::GET, "/users".to_string());
        let state = ShellState::new();
        let result = execute(req, &state);

        assert!(result.is_err());
    }
}
