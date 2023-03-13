use std::{mem, thread};
use crate::mm_format::MMFormat;
use crate::pcg::PCG;

mod helpers;
mod mm_format;
mod pcg;

type Deck = Vec<i32>;

fn deck(size: i32) -> Deck {
    (1..=size).collect()
}

fn pile_shuffle(deck: &mut Deck, pcg: &mut PCG, pile_count: usize, move_count: usize) {
    if pile_count <= 1 {
        return
    }
    let mut piles = vec![vec![]; pile_count];
    piles[0] = mem::take(deck);

    for _ in 0..move_count {
        let mut next_index = |prev_index, allow_empty| {
            let mut index;
            loop {
                index = pcg.rand_int(pile_count);
                if index != prev_index && (allow_empty || !piles[index].is_empty()) { break }
            }
            index
        };

        let from = next_index(pile_count, false);
        let to = next_index(from, true);

        fn from_to(piles: &mut [Deck], i: usize, j: usize, swap: bool) -> (&mut Deck, &mut Deck) {
            if i > j {
                from_to(piles, j, i, !swap)
            } else {
                let (from, tail) = piles[i..].split_first_mut().unwrap();
                let to = &mut tail[j - i - 1];
                if swap {(to, from)} else {(from, to)}
            }
        }

        let (from, to) = from_to(&mut piles, from, to, false);

        let cut_index = pcg.rand_int(1 + from.len());
        to.append(&mut from.split_off(cut_index));
    }

    let (last, rest) = piles.split_last_mut().unwrap();
    *deck = mem::take(last);

    for vec in rest.iter_mut().rev() {
        deck.append(vec);
    }
}

fn riffle_shuffle(deck: &mut Deck, pcg: &mut PCG, riffle_count: usize) {
    let size = deck.len();

    for _ in 0..riffle_count {
        let flips = (0..size).map(|_| pcg.rand_bool()).collect::<Vec<_>>();
        let heads = flips.iter().filter(|x| **x).count();

        let mut first = mem::take(deck);
        let second = first.split_off(heads);

        let mut first = first.into_iter();
        let mut second = second.into_iter();

        for i in 0..size {
            deck.push(if flips[i] {&mut first} else {&mut second}.next().unwrap());
        }
    }
}

fn weave_shuffle(deck: &mut Deck, weave_count: usize, out: bool) {
    let size = deck.len();
    let half = size / 2;
    let offsets = if out {(0, half)} else {(half, 0)};

    for _ in 0..weave_count {
        let mut other = vec![0; size];
        for i in 0..half {
            other[2 * i] = deck[i + offsets.0];
            other[2 * i + 1] = deck[i + offsets.1];
        }
        *deck = other;
    }
}

fn entropy_bits(deck: &Deck) -> f64 {
    let size = deck.len();
    let mut p = vec![0; size];

    let mut increment = |i, j| p[i32::rem_euclid(deck[i] - deck[j], size as i32) as usize] += 1;

    increment(0, size - 1);
    for i in 1..size {
        increment(i, i - 1);
    }

    p.iter().filter_map(|x| {
        match *x {
            0 => None,
            x => Some(x as f64 / size as f64),
        }
    }).fold(0.0, |acc, x| acc - x * x.log2())
}

fn shuffle_stats<T>(size: i32, repeats: usize, mut shuffle: T) -> (f64, f64)
    where T: FnMut(&mut Deck, &mut PCG)
{
    let mut pcg = PCG::new();
    let mut results = vec![0.0; repeats];

    for i in 0..repeats {
        let mut deck = deck(size);
        shuffle(&mut deck, &mut pcg);
        results[i] = entropy_bits(&deck);
    }

    let mean = results.iter().sum::<f64>() / repeats as f64;
    let sigma = (results.iter().map(|x| {
        let diff = x - mean;
        diff * diff
    }).sum::<f64>() / repeats as f64).sqrt();

    (mean, sigma)
}

fn main() {
    let size = 52;
    let repeats = 1000;

    let max_piles = 13;
    let max_moves = 250;

    let mut threads = vec![];

    for pile_count in 1..=max_piles {
        for move_count in 1..=max_moves {
            threads.push(thread::spawn(move || {
                shuffle_stats(size, repeats, |deck, pcg| {
                    pile_shuffle(deck, pcg, pile_count, move_count)
                })
            }))
        }
    }

    println!("Pile shuffling...");
    let mut mm_string = String::new();
    helpers::vec_chunks(helpers::join_all(threads), max_moves).mm_format(&mut mm_string);
    helpers::file_write("pile_shuffle_data.txt", &mm_string);
    println!("Finished!");

    let max_riffles = 10;

    let mut threads = vec![];

    for riffle_count in 0..=max_riffles {
        threads.push(thread::spawn(move || {
            shuffle_stats(size, repeats, |deck, pcg| {
                riffle_shuffle(deck, pcg, riffle_count);
            })
        }))
    }

    println!("Riffle shuffling...");
    let mut mm_string = String::new();
    helpers::vec_chunks(helpers::join_all(threads), 1 + max_riffles).mm_format(&mut mm_string);
    helpers::file_write("riffle_shuffle_data.txt", &mm_string);
    println!("Finished!");

    let max_weaves = 52;

    let mut threads = vec![];

    let mut weave = |out| for weave_count in 0..=max_weaves {
        threads.push(thread::spawn(move || {
            shuffle_stats(size, 1, |deck, _| {
                weave_shuffle(deck, weave_count, out);
            })
        }))
    };

    weave(false);
    weave(true);

    println!("Weave shuffling...");
    let mut mm_string = String::new();
    helpers::vec_chunks(helpers::join_all(threads), 1 + max_weaves).mm_format(&mut mm_string);
    helpers::file_write("weave_shuffle_data.txt", &mm_string);
    println!("Finished!");
}
