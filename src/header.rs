use std::io;
use colored::Colorize;

pub fn presentation() {
    let text = r#"
    _____ ______       ___    ___      ___  ________   ________ ________     
    |\   _ \  _   \    |\  \  /  /|    |\  \|\   ___  \|\  _____\\   __  \    
    \ \  \\\__\ \  \   \ \  \/  / /    \ \  \ \  \\ \  \ \  \__/\ \  \|\  \   
     \ \  \\|__| \  \   \ \    / /      \ \  \ \  \\ \  \ \   __\\ \  \\\  \  
      \ \  \    \ \  \   \/  /  /        \ \  \ \  \\ \  \ \  \_| \ \  \\\  \ 
       \ \__\    \ \__\__/  / /           \ \__\ \__\\ \__\ \__\   \ \_______\
        \|__|     \|__|\___/ /             \|__|\|__| \|__|\|__|    \|_______|
                      \|___|/                                                                                                                   
    "#;

    println!("{}", text.yellow());

    println!("🧰 In this project you will see some information about yourself through the GitHub user");
    println!("🔨 João Lucas");
    println!("💻 https://github.com/joaolfp/MyInfo");
    println!("🔢 0.5.0 Version \n");
}

pub fn header() -> String {
    println!("Enter the user you want to query:");
    let mut user = String::new();
    io::stdin().read_line(&mut user).expect("Error typing username");
    println!("---------------------------------");
    return user
}