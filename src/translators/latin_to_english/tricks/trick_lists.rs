use crate::translators::latin_to_english::tricks::Operation;

pub struct Trick {
    pub operation: Operation,
    pub str_1: &'static str,
    pub str_2: &'static str,
}

pub fn match_tricks_list(first_char_of_word: char) -> Vec<Trick> {
    match first_char_of_word {
        'a' => get_a_tricks(),
        'd' => get_d_tricks(),
        'e' => get_e_tricks(),
        'f' => get_f_tricks(),
        'g' => get_g_tricks(),
        'h' => get_h_tricks(),
        'i' => get_i_tricks(),
        'j' => get_j_tricks(),
        'k' => get_k_tricks(),
        'l' => get_l_tricks(),
        'm' => get_m_tricks(),
        'n' => get_n_tricks(),
        'o' => get_o_tricks(),
        'p' => get_p_tricks(),
        's' => get_s_tricks(),
        't' => get_t_tricks(),
        'u' => get_u_tricks(),
        'y' => get_y_tricks(),
        'z' => get_z_tricks(),
        _ => panic!("Invalid first char of word: {}", first_char_of_word),
    }
}

pub fn match_slur_trick_list(first_char_of_word: char) -> Vec<Trick> {
    match first_char_of_word {
        'a' => get_a_slur_tricks(),
        'c' => get_c_slur_tricks(),
        'i' => get_i_slur_tricks(),
        'n' => get_n_slur_tricks(),
        'q' => get_q_slur_tricks(),
        's' => get_s_slur_tricks(),
        _ => panic!("Invalid first char of word: {}", first_char_of_word),
    }
}

fn get_a_tricks() -> Vec<Trick> {
    let a_trick_list: Vec<Trick> = vec![
        Trick {
            operation: Operation::FlipFlop,
            str_1: "adgn",
            str_2: "agn",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "adsc",
            str_2: "asc",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "adsp",
            str_2: "asp",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "arqui",
            str_2: "arci",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "arqu",
            str_2: "arcu",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "ae",
            str_2: "e",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "al",
            str_2: "hal",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "am",
            str_2: "ham",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "ar",
            str_2: "har",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "aur",
            str_2: "or",
        },
    ];

    a_trick_list
}

fn get_d_tricks() -> Vec<Trick> {
    let b_trick_list: Vec<Trick> = vec![
        Trick {
            operation: Operation::Flip,
            str_1: "dampn",
            str_2: "damn",
        },
        // OLD p.52
        Trick {
            operation: Operation::FlipFlop,
            str_1: "dis",
            str_2: "disj",
        },
        // OLD p.55
        Trick {
            operation: Operation::FlipFlop,
            str_1: "dir",
            str_2: "disr",
        },
        // OLD p.52
        Trick {
            operation: Operation::FlipFlop,
            str_1: "dir",
            str_2: "der",
        },
        // OLD p.507/52
        Trick {
            operation: Operation::FlipFlop,
            str_1: "del",
            str_2: "dil",
        },
    ];

    b_trick_list
}

fn get_e_tricks() -> Vec<Trick> {
    let e_trick_list: Vec<Trick> = vec![
        Trick {
            operation: Operation::FlipFlop,
            str_1: "ecf",
            str_2: "eff",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "ecs",
            str_2: "exs",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "es",
            str_2: "ess",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "ex",
            str_2: "exs",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "eid",
            str_2: "id",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "el",
            str_2: "hel",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "e",
            str_2: "ae",
        },
    ];

    e_trick_list
}

// try lead then all
fn get_f_tricks() -> Vec<Trick> {
    let f_trick_list: Vec<Trick> = vec![
        Trick {
            operation: Operation::FlipFlop,
            str_1: "faen",
            str_2: "fen",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "faen",
            str_2: "foen",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "fed",
            str_2: "foed",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "fe",
            str_2: "foet",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "f",
            str_2: "ph",
        },
    ];

    f_trick_list
}

fn get_g_tricks() -> Vec<Trick> {
    let g_trick_list = vec![Trick {
        operation: Operation::Flip,
        str_1: "gna",
        str_2: "na",
    }];

    g_trick_list
}

fn get_h_tricks() -> Vec<Trick> {
    let h_trick_list = vec![
        Trick {
            operation: Operation::Flip,
            str_1: "har",
            str_2: "ar",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "hal",
            str_2: "al",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "ham",
            str_2: "am",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "hel",
            str_2: "el",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "hol",
            str_2: "ol",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "hum",
            str_2: "um",
        },
    ];

    h_trick_list
}

fn get_i_tricks() -> Vec<Trick> {
    let i_trick_list = vec![Trick {
        operation: Operation::Flip,
        str_1: "i",
        str_2: "j",
    }];

    i_trick_list
}

fn get_j_tricks() -> Vec<Trick> {
    let j_trick_list = vec![Trick {
        operation: Operation::Flip,
        str_1: "j",
        str_2: "i",
    }];

    j_trick_list
}

fn get_k_tricks() -> Vec<Trick> {
    let k_trick_list = vec![
        Trick {
            operation: Operation::Flip,
            str_1: "k",
            str_2: "c",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "c",
            str_2: "k",
        },
    ];

    k_trick_list
}

fn get_l_tricks() -> Vec<Trick> {
    let l_trick_list = vec![Trick {
        operation: Operation::FlipFlop,
        str_1: "lub",
        str_2: "lib",
    }];

    l_trick_list
}

fn get_m_tricks() -> Vec<Trick> {
    let m_trick_list = vec![Trick {
        operation: Operation::FlipFlop,
        str_1: "mani",
        str_2: "manu",
    }];

    m_trick_list
}

fn get_n_tricks() -> Vec<Trick> {
    let n_trick_list = vec![
        Trick {
            operation: Operation::Flip,
            str_1: "na",
            str_2: "gna",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "nihil",
            str_2: "nil",
        },
    ];

    n_trick_list
}

fn get_o_tricks() -> Vec<Trick> {
    let o_trick_list = vec![
        Trick {
            operation: Operation::FlipFlop,
            str_1: "obt",
            str_2: "opt",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "obs",
            str_2: "ops",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "ol",
            str_2: "hol",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "opp",
            str_2: "op",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "or",
            str_2: "aur",
        },
    ];

    o_trick_list
}

fn get_p_tricks() -> Vec<Trick> {
    let p_trick_list = vec![
        Trick {
            operation: Operation::Flip,
            str_1: "ph",
            str_2: "f",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "pre",
            str_2: "prae",
        },
    ];

    p_trick_list
}

// From Oxford Latin Dictionary p.1815 "sub-"
fn get_s_tricks() -> Vec<Trick> {
    let s_trick_list = vec![
        Trick {
            operation: Operation::FlipFlop,
            str_1: "subsc",
            str_2: "susc",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "subsp",
            str_2: "susp",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "subc",
            str_2: "susc",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "succ",
            str_2: "susc",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "subt",
            str_2: "supt",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "subt",
            str_2: "sust",
        },
    ];

    s_trick_list
}

fn get_t_tricks() -> Vec<Trick> {
    let t_trick_list = vec![Trick {
        operation: Operation::FlipFlop,
        str_1: "transv",
        str_2: "trav",
    }];

    t_trick_list
}

// u is not v for this purpose
fn get_u_tricks() -> Vec<Trick> {
    let u_trick_list = vec![
        Trick {
            operation: Operation::Flip,
            str_1: "ul",
            str_2: "hul",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "uol",
            str_2: "vul",
        },
    ];

    u_trick_list
}

fn get_y_tricks() -> Vec<Trick> {
    let y_trick_list = vec![Trick {
        operation: Operation::Flip,
        str_1: "y",
        str_2: "i",
    }];

    y_trick_list
}

fn get_z_tricks() -> Vec<Trick> {
    let z_trick_list = vec![Trick {
        operation: Operation::Flip,
        str_1: "z",
        str_2: "di",
    }];

    z_trick_list
}

fn get_a_slur_tricks() -> Vec<Trick> {
    let a_slur_trick_list = vec![
        Trick {
            operation: Operation::FlipFlop,
            str_1: "abs",
            str_2: "aps",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "acq",
            str_2: "adq",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "ante",
            str_2: "anti",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "auri",
            str_2: "aure",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "auri",
            str_2: "auru",
        },
    ];

    a_slur_trick_list
}

fn get_c_slur_tricks() -> Vec<Trick> {
    let c_slur_trick_list = vec![
        Trick {
            operation: Operation::Flip,
            str_1: "circum",
            str_2: "circun",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "con",
            str_2: "com",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "co",
            str_2: "com",
        },
        Trick {
            operation: Operation::Flip,
            str_1: "co",
            str_2: "con",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "conl",
            str_2: "coll",
        },
    ];

    c_slur_trick_list
}

// for some forms of eo the "i" stem grates with an "is .. ." ending
fn get_i_slur_tricks() -> Vec<Trick> {
    let i_slur_trick_list = vec![
        Trick {
            operation: Operation::FlipFlop,
            str_1: "inb",
            str_2: "imb",
        },
        Trick {
            operation: Operation::FlipFlop,
            str_1: "inp",
            str_2: "imp",
        },
    ];

    i_slur_trick_list
}

fn get_n_slur_tricks() -> Vec<Trick> {
    let n_slur_trick_list = vec![Trick {
        operation: Operation::Flip,
        str_1: "non",
        str_2: "nun",
    }];

    n_slur_trick_list
}

fn get_q_slur_tricks() -> Vec<Trick> {
    let q_slur_trick_list = vec![Trick {
        operation: Operation::FlipFlop,
        str_1: "quadri",
        str_2: "quadru",
    }];

    q_slur_trick_list
}

fn get_s_slur_tricks() -> Vec<Trick> {
    let s_slur_trick_list = vec![
        // Latham
        Trick {
            operation: Operation::Flip,
            str_1: "se",
            str_2: "ce",
        },
    ];

    s_slur_trick_list
}

pub fn get_any_tricks() -> Vec<Trick> {
    let any_trick_list = vec![
        Trick {
            operation: Operation::Internal,
            str_1: "ae",
            str_2: "e",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "bul",
            str_2: "bol",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "bol",
            str_2: "bul",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "cl",
            str_2: "cul",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "cu",
            str_2: "quu",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "f",
            str_2: "ph",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "ph",
            str_2: "f",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "h",
            str_2: "",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "oe",
            str_2: "e",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "vul",
            str_2: "vol",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "uol",
            str_2: "vul",
        },
    ];

    any_trick_list
}

pub fn get_medieval_tricks() -> Vec<Trick> {
    let medieval_trick_list = vec![
        // Harrington/Elliott    1.1.1
        Trick {
            operation: Operation::Internal,
            str_1: "col",
            str_2: "caul",
        },
        // Harrington/Elliott    1.3
        Trick {
            operation: Operation::Internal,
            str_1: "e",
            str_2: "ae",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "o",
            str_2: "u",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "i",
            str_2: "y",
        },
        // Harrington/Elliott    1.3.1
        Trick {
            operation: Operation::Internal,
            str_1: "ism",
            str_2: "sm",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "isp",
            str_2: "sp",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "ist",
            str_2: "st",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "iz",
            str_2: "z",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "esm",
            str_2: "sm",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "esp",
            str_2: "sp",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "est",
            str_2: "st",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "ez",
            str_2: "z",
        },
        // Harrington/Elliott    1.4
        Trick {
            operation: Operation::Internal,
            str_1: "di",
            str_2: "z",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "f",
            str_2: "ph",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "is",
            str_2: "ix",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "b",
            str_2: "p",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "d",
            str_2: "t",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "v",
            str_2: "b",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "v",
            str_2: "f",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "s",
            str_2: "x",
        },
        // Harrington/Elliott    1.4.1
        Trick {
            operation: Operation::Internal,
            str_1: "ci",
            str_2: "ti",
        },
        // Harrington/Elliott    1.4.2
        Trick {
            operation: Operation::Internal,
            str_1: "nt",
            str_2: "nct",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "s",
            str_2: "ns",
        },
        // Other
        Trick {
            operation: Operation::Internal,
            str_1: "ch",
            str_2: "c",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "c",
            str_2: "ch",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "th",
            str_2: "t",
        },
        Trick {
            operation: Operation::Internal,
            str_1: "t",
            str_2: "th",
        },
    ];

    medieval_trick_list
}
