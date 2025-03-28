use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    // P95
    // let suit = Suit::Club;
    // let rank = 1;
    // let card = Card { suit, rank };
    // println!("{:?}", card);

    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }

    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    // println!("{:?}", deck);

    // P96
    let mut hand: Vec<Card> = Vec::new();
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // println!("---Hand---");
    // for (i, card) in hand.iter().enumerate() {
    //     println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    // }
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    println!("---Hand---");
    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }

    // P98
    println!("入れ替えたいカードの番号を入力してください(例: 1 2 4)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap()) // 文字列を数値に変換
        .collect::<Vec<usize>>(); // Vecに変換

    println!("{:?}", numbers);
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap(); // collectionのindexは0から始まるので-1
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    println!("---Hand---");
    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }

    // P100
    let suit = hand.first().unwrap().suit;
    let flash = hand.iter().all(|card| card.suit == suit);
    // ペア数のチェック
    let mut count = 0;
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if flash {
        println!("フラッシュです");
    } else if count == 4 {
        println!("フォーカードです");
    } else if count == 3 {
        println!("スリーカードです");
    } else if count == 2 {
        println!("ツーペアです");
    } else if count == 1 {
        println!("ワンペアです");
    } else {
        println!("役なし...");
    }
}
