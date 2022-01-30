#[derive(Default)]
pub struct Alice {
    date: bool,
}

#[derive(Default)]
pub struct Bob {
    date: bool
}

pub trait Player {
    fn encode(&mut self, date: bool) -> Deck;
}

impl Player for Alice {

    fn encode(&mut self, date: bool) -> Deck {
        self.date = date;
        let cards = if date {
            vec![Card::Queen, Card::King]
        } else {
            vec![Card::King, Card::Queen]
        };
        Deck { cards }
    }
}

impl Player for Bob {

    fn encode(&mut self, date: bool) -> Deck {
        self.date = date;
        let cards = if date {
            vec![Card::King, Card::Queen]
        } else {
            vec![Card::Queen, Card::King]
        };
        Deck { cards }
    }
}


pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {

    pub fn join(mut left: Deck, mut right: Deck) -> Deck {
        left.cards.push(Card::King);
        left.cards.append(&mut right.cards);
        left
    }

    pub fn cyclic_shift(mut deck: Deck, shift: usize) -> Deck {
        deck.cards.rotate_left(shift);
        deck
    }

    pub fn decode(deck: Deck) -> bool {
        if deck.cards[0] == Card::Queen && deck.cards[4] == Card::Queen {
            return true;
        }
        let mut contiguous = false;
        for card in deck.cards {
            if card == Card::Queen {
                if contiguous {
                    return true;
                } else {
                    contiguous = true;
                }
            } else {
                contiguous = false;
            }
        }
        false
    }
}

#[derive(PartialEq)]
pub enum Card {
    Queen,
    King,
}


