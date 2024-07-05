use colored::Colorize;

pub fn banner() {
    println!("{}", r"________                __
\______ \ _____ _______|  | __ ____ ___.__. ____
 |    |  \\__  \\_  __ \  |/ // __ <   |  |/ __ \
 |    `   \/ __ \|  | \/    <\  ___/\___  \  ___/
/_______  (____  /__|  |__|_ \\___  > ____|\___  >
        \/     \/           \/    \/\/         \/ ".blue());
}

pub fn copyright() {
    println!("{}", "\nhttps://github.com/yq93dskimzm2/Darkeye\n".bright_cyan())
}
