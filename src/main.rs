use std::fmt::format;

use rand::{self, seq::{self, SliceRandom}, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {

    fn new() -> Self {
        Deck {
            cards: Deck::build_deck(),
        }
    }

    fn build_deck() -> Vec<String>  {
        let suits = ["Hearts", "Diamonds", "Spades", "Club"];
        let rank_cards =["King", "Queen", "Joker"];
        let rank_start = 1;
        let rank_end = 13;

        let mut cards: Vec<String> = Vec::new();

        for suit in &suits {
            for rank_card in &rank_cards {
                cards.push(format!("{:#?} of {:#?}", rank_card, suit))
            }
            for rank in rank_start..=rank_end{
                cards.push(format!("{:?} of {:?}", rank, suit))
            }
        }

        cards
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
}

fn main() {

    let mut deck = Deck::new();

    let mut len = deck.cards.len();

    println!("The number of cards in the current deck is: {:?}", len);
    println!("The deck: {:?}", deck);

    deck.shuffle();
    println!("Shuffling deck....\n Deck = {:#?}", deck);

    Deck::build_deck();

}
 