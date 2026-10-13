#[allow(unused)]
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

#[allow(unused)]
fn rustlatin(sentence: &str) -> String {
    let collection: Vec<_> = sentence
        .split(' ')
        .into_iter()
        .map(|word| {
            let first_char = word.chars().next().unwrap_or_default();
            if VOWELS.contains(&first_char.to_ascii_lowercase()) {
                "sr".to_string() + word
            } else {
                word.to_string() + "rs"
            }
        }).collect();


    collection.join(" ")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_works(){
        let mystring = rustlatin("apple Ios Tae wew");
        println!("{}", mystring);
    }
}