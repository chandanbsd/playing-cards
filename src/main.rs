#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {

    let deck = Deck {
        cards: Vec::new(),
    };

    let a = deck.cards.len();

    println!("The number of cards in the current deck is: {:?}", a);
    println!("The deck: {:?}", deck);

}
 