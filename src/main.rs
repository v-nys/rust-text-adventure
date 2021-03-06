use std::io;

struct Item(&'static str);

struct Location {
    description: &'static str,
    items: Vec<Item>,
}

// how to do actions?
// as an enum? sounds reasonable.
// what properties do they have?
// a keyword which is typed in
// one or more arguments, possibly a variable number
// something which happens when they are invoked

// "look": print the area description
// "take + item": move the item to the player's inventory
// "go + direction": move to the area in that direction
// "use + item (+ object)": maybe remove the item, invoke a (targeted?) effect

// alternatively, instantiate actions at runtime
// write a struct for each one with arity and synonyms
// problem: each one is associated with a function of some sort
// do not want to go adding these at runtime if it can be avoided

fn parse(command: String) {
    let mut tokens = command.split(' ');
    let verb = tokens.nth(0);
    match verb {
        Some("look") => println!("You look around."),
        Some("take") => println!("You take something."),
        Some("go") => println!("You go somewhere."),
        Some("use") => println!("You use something."),
        Some("quit") => println!("Need CTRL-D for now."),
        _ => println!("Unknown command!"),
    }
    // look should just check for arity 0 invoke an associated function
    // take should parse rest of the command, see if there is an item with a name equal to the rest of the command
    // go should parse exactly one more token and go in that direction... how?
    // use should parse either one or two tokens and act accordingly
}

fn main() {
    let item1 = Item("Little key");
    let item2 = Item("Big key");
    let hall = Location{ description: "A wide open space. Streams of lava flow on both sides of the entry gate.",
                         items: vec![item1, item2] };
    loop {
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");
        parse(input);
    }
}
