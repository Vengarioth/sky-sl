mod command;

use command::*;

fn main() {
    match Command::from_args() {
        Command::Parse { file, output } => {
            let input = std::fs::read_to_string(file).expect("Could not read file");
            
            let token = sky_sl::lexer::tokenize(&input);
            let syntax_tree = sky_sl::parser::parse(&token);

            dbg!(syntax_tree);

            std::fs::write(output, "").expect("Could not write file");
        },
    }
}
