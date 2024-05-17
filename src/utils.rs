use colored::Colorize;

pub fn banner() {
    println!("{}", r"________                __
\______ \ _____ _______|  | __ ____ ___.__. ____
 |    |  \\__  \\_  __ \  |/ // __ <   |  |/ __ \
 |    `   \/ __ \|  | \/    <\  ___/\___  \  ___/
/_______  (____  /__|  |__|_ \\___  > ____|\___  >
        \/     \/           \/    \/\/         \/ ".red());
}

pub fn copyright() {
    println!("{}", "\nhttps://github.com/v7sr14ul2x9/Darkeye\n".bright_cyan())
}