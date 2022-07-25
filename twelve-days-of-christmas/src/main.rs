use std::io;

fn main() {
    loop {
        let mut n = String::new();

        println!(
            "Enter \"12\" to print the lyrics to the Christmas carol \"The Twelve Days of Christmas\"."
        );

        io::stdin().read_line(&mut n).expect("Failed to read line");

        if n.trim() == "12" {
            println!("Queuing...");
            queue_lyrics();
            break;
        }
    }
}

fn queue_lyrics() {
    let a = [
        "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th",
    ];
    let mut remaining = 11;

    println!("On the 1st day of Christmas");
    println!("My true love sent to me");
    println!("A partridge in a pear tree!\n");

    for number in a {
        println!("On the {number} day of Christmas");
        println!("My true love sent to me");

        while remaining < 2 {
            println!("Twelve drummers drummin'");
            break;
        }
        while remaining < 3 {
            println!("Eleven pipers pipin'");
            break;
        }
        while remaining < 4 {
            println!("Ten lords a leapin'");
            break;
        }
        while remaining < 5 {
            println!("Nine ladies dancin'");
            break;
        }
        while remaining < 6 {
            println!("Eight maids a milkin'");
            break;
        }
        while remaining < 7 {
            println!("Seven swans a swimmin'");
            break;
        }
        while remaining < 8 {
            println!("Six geese a layin'");
            break;
        }
        while remaining < 9 {
            println!("Five golden rings!");
            break;
        }
        while remaining < 10 {
            println!("Four calling birds");
            break;
        }
        while remaining < 11 {
            println!("Three french hens");
            break;
        }
        while remaining < 12 {
            println!("Two turtle doves");
            break;
        }

        remaining -= 1;
        println!("And a partridge in a pear tree!\n");
    }
}

// On the first day of Christmas
// My true love sent to me
// A partridge in a pear tree!

// Two turtle doves
// On the second day of Christmas
// My true love sent to me
// Two turtle doves
// And a partridge in a pear tree!

// Three french hens
// On the third day of Christmas
// My true love sent to me
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Four calling birds
// On the fourth day of Christmas (what's a calling bird)
// My true love sent to me
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Five golden rings!
// On the fifth day of Christmas
// My true love sent to me
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Six geese a layin'
// On the sixth day of Christmas
// My true love sent to me
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Seven swans a swimmin'
// On the seventh day of Christmas
// My true love sent to me
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Eight maids a milkin'
// On the eighth day of Christmas
// My true love sent to me
// Eight maids a milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds (calling birds)
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Nine ladies dancin'
// On the ninth day of christmas
// My true love sent to me
// Nine ladies dancin'
// Eight maids a milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Ten lords a leapin'
// On the tenth day of Christmas
// My true love sent to me
// Ten lords a leapin'
// Nine ladies dancin'
// Eight maids a milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Eleven pipers pipin'
// On the eleventh day of Christmas
// My true love sent to me
// Eleven pipers pipin'
// Ten lords a leapin'
// Nine ladies dancin'
// Eight maids a milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

// Twelve drummers drummin'
// On the twelfth day of Christmas
// My true love sent to me
// Twelve drummers drummin'
// Eleven pipers pipin'
// Ten lords a leapin'
// Nine ladies dancin'
// Eight maids milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
