use crate::utils::tricks::tricks::{Operation, Trick};

pub fn get_tricks_list(first_char_of_word: char) -> Vec<Trick> {
    match first_char_of_word {
        'a' => return get_a_tricks(),
        'd' => return get_d_tricks(),
        _ => panic!("Invalid first char of word: {}", first_char_of_word),
    }
}

pub fn get_a_tricks() -> Vec<Trick> {
    let a_trick_list: Vec<Trick> = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "adgn",
            flip_flop2: "agn",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "adsc",
            flip_flop2: "asc",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "adsp",
            flip_flop2: "asp",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "arqui",
            flip_flop2: "arci",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "arqu",
            flip_flop2: "arcu",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "ae",
            flip_flip4: "e",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "al",
            flip_flip4: "hal",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "am",
            flip_flip4: "ham",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "ar",
            flip_flip4: "har",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "aur",
            flip_flip4: "or",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    a_trick_list
}

fn get_d_tricks() -> Vec<Trick> {
    let b_trick_list: Vec<Trick> = vec![Trick {
        max_attempts: 0,
        operation: Operation::FlipFlop,
        flip_flop1: "",
        flip_flop2: "",
        flip_flip3: "dampn",
        flip_flip4: "damn",
        internal1: "",
        internal2: "",
        slur1: "",
    }];

    b_trick_list
}
