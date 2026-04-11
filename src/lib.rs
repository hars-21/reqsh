pub mod context;
pub mod help;
pub mod helper;
pub mod parser;

use crate::context::RequestContext;
use std::process::Command;

pub fn get_request(url: &str, ctx: &mut RequestContext) -> String {
    let full_url = if let Some(base) = ctx.get_base_url()
        && url.starts_with('/')
    {
        format!("{}{}", base, url)
    } else {
        url.to_string()
    };

    let response = Command::new("curl")
        .arg(&full_url)
        .output()
        .map_err(|err| err.to_string())
        .unwrap();

    if response.status.success() {
        String::from_utf8(response.stdout).unwrap().to_string()
    } else {
        String::from_utf8(response.stderr).unwrap().to_string()
    }
}
