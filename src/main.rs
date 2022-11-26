use std::io;
use std::io::Write;

fn main() {
    let days = [
        "first", "second", "third",    "fourth",
        "fifth", "sixth",  "seventh",  "eighth",
        "ninth", "tenth",  "eleventh", "twelfth"
    ];
    let mut day = 0;
    let mut gifts: String = String::from("");
    while day < 12 {
        let today = days[day];
        println!("On the {} day of Christmas, my true love sent to me", today);
        match day+1 {
            1 => {
                print!("A ");
                io::stdout().flush().unwrap();
            },
            2 => {
                gifts = String::from("Two turtledoves");
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            3 => {
                let newgift = String::from("Three French hens\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            4 => {
                let newgift = String::from("Four calling birds\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            5 => {
                let newgift = String::from("Five gold rings (five golden rings)\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            6 => {
                let newgift = String::from("Six geese a-laying\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            7 => {
                let newgift = String::from("Seven swans a-swimming\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            8 => {
                let newgift = String::from("Eight maids a-milking\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            9 => {
                let newgift = String::from("Nine ladies dancing\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            10 => {
                let newgift = String::from("Ten lords a-leaping\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            11 => {
                let newgift = String::from("Eleven pipers piping\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            12 => {
                let newgift = String::from("Twelve drummers drumming\n");
                gifts = newgift + &gifts;
                println!("{}", gifts);
                println!("And a partridge in a pear tree");
                print!("And a ");
                io::stdout().flush().unwrap();
            },
            _ => {}
        };
        println!("partridge in a pear tree\n");
        day += 1;
    }
}
