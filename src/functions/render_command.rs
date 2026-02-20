use std::process::Command;

// TODO: This function doesn't escape the spaces in program or args
pub fn render_command(command: &Command) -> String {
    let mut output = vec![command.get_program()];
    output.extend(command.get_args());
    output.join(" ".as_ref()).to_string_lossy().into()
}
