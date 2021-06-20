mod command;

use command::*;

fn main() {
    match Command::from_args() {
        Command::Parse { file, .. } => {
            let input = std::fs::read_to_string(file).expect("Could not read file");
            
            let token = sky_sl::lexer::tokenize(&input);
            let result = sky_sl::parser::parse(&token, &input);

            dbg!(result.errors());
            dbg!(result.tree());
        },
    }
}
