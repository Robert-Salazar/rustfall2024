

fn main() {
    let new_file: &str = "hello.txt"
    let command_to_execute: String = format!("touch {}", new_file);
    println!("{}", command_to_execute);

    let output: &mut Command =command::new(program: "sh").arg("-c").arg(command_to_execute).spawn();
}
