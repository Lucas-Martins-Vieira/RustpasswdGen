use rand::Rng;
use rand::distributions::Slice;

fn main() {

let characters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '#', '$', '%', '&', '*', '/', '?' ];
let characters_dist = Slice::new(&characters).unwrap();
let rng = rand::thread_rng();

    //Build a 16 character password

let characters_string: String = rng
    .sample_iter(&characters_dist)
    .take(16)
    .collect();

println!("{}", characters_string);
}
