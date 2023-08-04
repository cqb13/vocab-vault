use std::vec;

use crate::utils::tricks::tricks::Operation;

pub struct Trick {
    pub max_attempts: i32,
    pub operation: Operation,
    pub flip_flop1: &'static str,
    pub flip_flop2: &'static str,
    pub flip_flip3: &'static str,
    pub flip_flip4: &'static str,
    pub internal1: &'static str,
    pub internal2: &'static str,
    pub slur1: &'static str,
}

pub fn get_tricks_list(first_char_of_word: char) -> Vec<Trick> {
    match first_char_of_word {
        'a' => return get_a_tricks(),
        'd' => return get_d_tricks(),
        'e' => return get_e_tricks(),
        'f' => return get_f_tricks(),
        'g' => return get_g_tricks(),
        'h' => return get_h_tricks(),
        'k' => return get_k_tricks(),
        'l' => return get_l_tricks(),
        'm' => return get_m_tricks(),
        'n' => return get_n_tricks(),
        'o' => return get_o_tricks(),
        'p' => return get_p_tricks(),
        's' => return get_s_tricks(),
        't' => return get_t_tricks(),
        'u' => return get_u_tricks(),
        'y' => return get_y_tricks(),
        'z' => return get_z_tricks(),
        _ => panic!("Invalid first char of word: {}", first_char_of_word),
    }
}

fn get_a_tricks() -> Vec<Trick> {
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
    let b_trick_list: Vec<Trick> = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "dampn",
            flip_flip4: "damn",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        // OLD p.54
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "dis",
            flip_flop2: "disj",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        // OLD p.55
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "dir",
            flip_flop2: "disr",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        // OLD p.54
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "dir",
            flip_flop2: "der",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        // OLD p.507/54
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "del",
            flip_flop2: "dil",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    b_trick_list
}

fn get_e_tricks() -> Vec<Trick> {
    let e_trick_list: Vec<Trick> = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "ecf",
            flip_flop2: "eff",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "ecs",
            flip_flop2: "exs",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "es",
            flip_flop2: "ess",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "ex",
            flip_flop2: "exs",
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
            flip_flip3: "eid",
            flip_flip4: "id",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "el",
            flip_flip4: "hel",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "e",
            flip_flip4: "ae",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    e_trick_list
}

// try lead then all
fn get_f_tricks() -> Vec<Trick> {
    let f_trick_list: Vec<Trick> = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "faen",
            flip_flop2: "fen",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "faen",
            flip_flop2: "foen",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "fed",
            flip_flop2: "foed",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "fe",
            flip_flop2: "foet",
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
            flip_flip3: "f",
            flip_flip4: "ph",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    f_trick_list
}

fn get_g_tricks() -> Vec<Trick> {
    let g_trick_list = vec![Trick {
        max_attempts: 0,
        operation: Operation::Flip,
        flip_flop1: "",
        flip_flop2: "",
        flip_flip3: "gna",
        flip_flip4: "na",
        internal1: "",
        internal2: "",
        slur1: "",
    }];

    g_trick_list
}

fn get_h_tricks() -> Vec<Trick> {
    let h_trick_list = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "har",
            flip_flip4: "ar",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "hal",
            flip_flip4: "al",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "ham",
            flip_flip4: "am",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "hel",
            flip_flip4: "el",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "hol",
            flip_flip4: "ol",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "hum",
            flip_flip4: "um",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    h_trick_list
}

fn get_k_tricks() -> Vec<Trick> {
    let k_trick_list = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "k",
            flip_flip4: "c",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "c",
            flip_flip4: "k",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    k_trick_list
}

fn get_l_tricks() -> Vec<Trick> {
    let l_trick_list = vec![Trick {
        max_attempts: 1,
        operation: Operation::FlipFlop,
        flip_flop1: "lub",
        flip_flop2: "lib",
        flip_flip3: "",
        flip_flip4: "",
        internal1: "",
        internal2: "",
        slur1: "",
    }];

    l_trick_list
}

fn get_m_tricks() -> Vec<Trick> {
    let m_trick_list = vec![Trick {
        max_attempts: 1,
        operation: Operation::FlipFlop,
        flip_flop1: "mani",
        flip_flop2: "manu",
        flip_flip3: "",
        flip_flip4: "",
        internal1: "",
        internal2: "",
        slur1: "",
    }];

    m_trick_list
}

fn get_n_tricks() -> Vec<Trick> {
    let n_trick_list = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "na",
            flip_flip4: "gna",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "nihil",
            flip_flop2: "nil",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    n_trick_list
}

fn get_o_tricks() -> Vec<Trick> {
    let o_trick_list = vec![
        Trick {
            max_attempts: 1,
            operation: Operation::FlipFlop,
            flip_flop1: "obt",
            flip_flop2: "opt",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 1,
            operation: Operation::FlipFlop,
            flip_flop1: "obs",
            flip_flop2: "ops",
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
            flip_flip3: "ol",
            flip_flip4: "hol",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 1,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "opp",
            flip_flip4: "op",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "or",
            flip_flip4: "aur",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    o_trick_list
}

fn get_p_tricks() -> Vec<Trick> {
    let p_trick_list = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "ph",
            flip_flip4: "f",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 1,
            operation: Operation::FlipFlop,
            flip_flop1: "pre",
            flip_flop2: "prae",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    p_trick_list
}

// From Oxford Latin Dictionary p.1835 "sub-"
fn get_s_tricks() -> Vec<Trick> {
    let s_trick_list = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "subsc",
            flip_flop2: "susc",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "subsp",
            flip_flop2: "susp",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "subc",
            flip_flop2: "susc",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "succ",
            flip_flop2: "susc",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "subt",
            flip_flop2: "supt",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::FlipFlop,
            flip_flop1: "subt",
            flip_flop2: "sust",
            flip_flip3: "",
            flip_flip4: "",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    s_trick_list
}

fn get_t_tricks() -> Vec<Trick> {
    let t_trick_list = vec![Trick {
        max_attempts: 0,
        operation: Operation::FlipFlop,
        flip_flop1: "transv",
        flip_flop2: "trav",
        flip_flip3: "",
        flip_flip4: "",
        internal1: "",
        internal2: "",
        slur1: "",
    }];

    t_trick_list
}

// u is not v for this purpose
fn get_u_tricks() -> Vec<Trick> {
    let u_trick_list = vec![
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "ul",
            flip_flip4: "hul",
            internal1: "",
            internal2: "",
            slur1: "",
        },
        Trick {
            max_attempts: 0,
            operation: Operation::Flip,
            flip_flop1: "",
            flip_flop2: "",
            flip_flip3: "uol",
            flip_flip4: "vul",
            internal1: "",
            internal2: "",
            slur1: "",
        },
    ];

    u_trick_list
}

fn get_y_tricks() -> Vec<Trick> {
    let y_trick_list = vec![Trick {
        max_attempts: 0,
        operation: Operation::Flip,
        flip_flop1: "",
        flip_flop2: "",
        flip_flip3: "y",
        flip_flip4: "i",
        internal1: "",
        internal2: "",
        slur1: "",
    }];

    y_trick_list
}

fn get_z_tricks() -> Vec<Trick> {
    let z_trick_list = vec![Trick {
        max_attempts: 0,
        operation: Operation::Flip,
        flip_flop1: "",
        flip_flop2: "",
        flip_flip3: "z",
        flip_flip4: "di",
        internal1: "",
        internal2: "",
        slur1: "",
    }];

    z_trick_list
}
