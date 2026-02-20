use std::process::Command;

pub fn render_command(command: &Command) -> String {
    let parts = core::iter::once(command.get_program().to_string_lossy())
        .chain(command.get_args().map(|arg| arg.to_string_lossy()))
        .collect::<Vec<_>>();
    let result = shlex::try_join(parts.iter().map(|x| x.as_ref()));
    match result {
        Ok(string) => string,
        Err(_) => command.get_program().to_string_lossy().into_owned(),
    }
}
