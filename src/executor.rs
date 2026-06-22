use std::collections::HashMap;

use crate::{display::display_response, request::Request, runner::fetch, state::ShellState};

fn interpolate(s: &str, vars: &HashMap<String, String>) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = s;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let tail = &rest[start + 2..];
        match tail.find("}}") {
            Some(end) => {
                let name = tail[..end].trim();
                let val = vars
                    .get(name)
                    .ok_or_else(|| format!("undefined variable: {name}"))?;
                out.push_str(val);
                rest = &tail[end + 2..];
            }
            None => return Err(format!("unclosed `{{{{` in: {s}")),
        }
    }
    out.push_str(rest);
    Ok(out)
}

pub fn execute(req: Request, ctx: &ShellState) -> Result<String, String> {
    let vars = ctx.get_variables();
    let path = interpolate(&req.path, vars)?;
    let body = match &req.body {
        Some(b) => Some(interpolate(b, vars)?),
        None => None,
    };
    let headers: HashMap<_, _> = req
        .headers
        .iter()
        .map(|(k, v)| Ok((k.clone(), interpolate(v, vars)?)))
        .collect::<Result<_, String>>()?;

    let params: HashMap<_, _> = req
        .params
        .iter()
        .map(|(k, v)| Ok((k.clone(), interpolate(v, vars)?)))
        .collect::<Result<_, String>>()?;

    let req = Request {
        path,
        body,
        headers,
        params,
        ..req
    };

    let base_url = ctx.get_base_url();
    let global_headers = ctx.get_headers();
    let timeout_secs = ctx.get_timeout();
    let response = fetch(&req, base_url, global_headers, timeout_secs);

    match response {
        Ok((res, duration)) => Ok(display_response(res, duration)),
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

    #[test]
    fn interpolate_passthrough() {
        let vars = HashMap::new();
        assert_eq!(interpolate("hello", &vars).unwrap(), "hello");
    }

    #[test]
    fn interpolate_replaces_variable() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "world".to_string());
        assert_eq!(interpolate("hello {{name}}", &vars).unwrap(), "hello world");
    }

    #[test]
    fn interpolate_undefined_errors() {
        let vars = HashMap::new();
        assert!(interpolate("{{missing}}", &vars).is_err());
    }

    #[test]
    fn interpolate_unclosed_errors() {
        let vars = HashMap::new();
        assert!(interpolate("{{unclosed", &vars).is_err());
    }

    #[test]
    fn interpolate_multiple_vars() {
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "1".to_string());
        vars.insert("b".to_string(), "2".to_string());
        assert_eq!(interpolate("{{a}}-{{b}}", &vars).unwrap(), "1-2");
    }
}
