use colored::Colorize;

pub fn startup(){
    let header : &str = 
    "
│   __   __      _                 _       ____        _           
│   |  \\/  | __ _| |_ ___ _   _ ___( )___  | __ )  ___ | |_           
│   | |\\/| |/ _` | __/ _ \\ | | / __|// __| |  _ \\ / _ \\| __|   
│   | |  | | (_| | ||  __/ |_| \\__ \\ \\__ \\ | |_) | (_) | |_  
│   |_|  |_|\\__,_|\\__\\___|\\__,_|___/ |___/ |____/ \\___/ \\__|  ";
    println!("{}",header.bold().truecolor(248, 137, 34));
    let help : &str =     
"├──────────────────────────────────────────────────────────────┤
│                                                               
│               Click Ctrl+C to terminate bot                   
│                                                              
├──────────────────────────────────────────────────────────────┤
│                                                              
│                     Messages Received:                        ";
    println!("{}", help.bold().truecolor(248, 137, 34));
}

pub fn formatted_msg(author : &str, contents : &str){
    println!("{}\n{}{} sent message with following content: \n{}{}",
        "│".truecolor(248, 137, 34),
        "├".truecolor(248, 137, 34),
        author.bold().truecolor(248, 137, 34),
        "├".truecolor(248, 137, 34),
        contents.italic().bold().green());
}
