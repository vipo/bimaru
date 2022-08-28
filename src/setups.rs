use im::{hashmap, HashMap};
use uuid::{uuid, Uuid};

pub type Setup = [[u8; 10]; 10];
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SetupFormat{
    pub create_format: CreateFormat,
    pub hint_format: HintFormat,
    pub setup: Setup
}
pub type Setups = HashMap<Uuid, SetupFormat>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HintFormat {
    List,
    Nested,
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CreateFormat {
    List,
    Nested
}

pub fn build_all() -> Setups {
    hashmap! {
        uuid!("5109c2b1-7c4d-4f56-9be2-f6675c968331") =>
            SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::Nested, setup: GAME_0},
        uuid!("dd8fb490-72c8-485b-aeea-537b9be34e4b") =>
            SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: GAME_1},
        uuid!("37073150-f43d-4609-94ec-dcbeffcb472a") =>
            SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::List,   setup: GAME_2},
        uuid!("3a7a8f44-b224-40ff-9c5c-58a1b60eab4b") =>
            SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: GAME_3},
        uuid!("81520eec-47d6-43da-a692-2926a3dc2871") =>
            SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::List,   setup: GAME_4},
        uuid!("63dac12b-2afa-49e0-b133-edce3955b49a") =>
            SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: GAME_5},
        uuid!("399165ec-72cc-43f2-ba41-6f19f89afcf3") =>
            SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::List,   setup: game_6()},
        uuid!("d8ad5555-cd70-4b51-9d73-93272950178d") =>
            SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: game_7()},
        uuid!("31f1c720-e0e7-47e7-be5c-a94d32e1088d") =>
            SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::List,   setup: game_8()},
        uuid!("37a5acdf-6d9b-4de1-b4f1-6647fbb6feb0") =>
            SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: game_9()},
        uuid!("a087ab92-a5bd-4e1f-b61d-da27c355279e") =>
            SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::List,   setup: game_10()},
        uuid!("a470fa68-cc2d-4295-80b7-411869b65ddb") =>
            SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: game_11()},
        uuid!("685023f9-ebcb-499c-96c8-5a88bfbfb14d") =>
            SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::List,   setup: game_12()},
        uuid!("bb63cffd-b5cc-4803-a03f-40922646d0d4") =>
            SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: game_13()},
        uuid!("0de28b51-e8ef-41d5-a1e6-131b51c4a638") =>
            SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::List,   setup: game_14()},
        uuid!("06d6bab1-ff17-4c9e-8861-e6ae4b227b86") =>
            SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: game_15()},
    }
}

pub const MAX_INDEX: usize = 9;
pub const MIN_INDEX: usize = 0;

// purely experimental, do not expose to students
pub const GAME_0: Setup = [
    [00, 07, 06, 05, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [08, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [09, 00, 00, 04, 03, 02, 01, 00, 00, 20],
    [10, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 19],
    [11, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [12, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 18],
    [00, 13, 14, 00, 15, 16, 00, 17, 00, 00],
];

pub const GAME_1: Setup = [
    [00, 00, 00, 00, 00, 04, 03, 02, 01, 00],
    [17, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 05, 00, 00, 00, 00],
    [00, 00, 19, 00, 00, 06, 00, 00, 00, 00],
    [18, 00, 00, 00, 00, 07, 00, 00, 00, 15],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 16],
    [00, 00, 00, 08, 00, 20, 00, 00, 00, 00],
    [00, 00, 00, 09, 00, 00, 00, 11, 12, 00],
    [00, 00, 00, 10, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 13, 14, 00],
];

pub const GAME_2: Setup = [
    [00, 00, 00, 00, 00, 00, 00, 00, 20, 00],
    [00, 00, 13, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 14, 00, 00, 10, 00, 07, 00, 00],
    [00, 00, 00, 00, 00, 09, 00, 06, 00, 00],
    [00, 15, 00, 00, 00, 08, 00, 05, 00, 00],
    [00, 16, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 01, 02, 03, 04, 00, 00],
    [00, 17, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 18, 00, 11, 12, 00, 00, 00, 19, 00],
];

pub const GAME_3: Setup = [
    [00, 00, 00, 00, 00, 00, 00, 00, 16, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 15, 00],
    [19, 00, 00, 00, 00, 00, 12, 00, 00, 00],
    [00, 00, 13, 14, 00, 00, 11, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 08],
    [18, 00, 00, 00, 20, 00, 04, 00, 00, 09],
    [00, 00, 00, 00, 00, 00, 03, 00, 00, 10],
    [00, 00, 07, 06, 05, 00, 02, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 01, 00, 17, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
];

pub const GAME_4: Setup = [
    [00, 00, 00, 00, 00, 00, 00, 19, 00, 20],
    [00, 00, 01, 02, 03, 04, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 07, 06, 05, 00, 18, 00, 17],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [08, 09, 10, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 11, 00, 00, 00, 13, 00, 00, 15],
    [00, 00, 12, 00, 00, 00, 14, 00, 00, 16],
];

pub const GAME_5: Setup = [
    [20, 00, 00, 00, 00, 00, 00, 00, 00, 19],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 01, 00, 05, 06, 07, 00, 00, 00, 00],
    [00, 02, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 03, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 04, 00, 00, 15, 16, 00, 00, 14, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 13, 00],
    [00, 00, 00, 17, 00, 00, 10, 00, 00, 00],
    [00, 18, 00, 00, 00, 00, 09, 00, 11, 00],
    [00, 00, 00, 00, 00, 00, 08, 00, 12, 00],
];

pub fn game_6() -> Setup {
    transpose(GAME_1)
}

pub fn game_7() -> Setup {
    transpose(GAME_2)
}

pub fn game_8() -> Setup {
    transpose(GAME_3)
}

pub fn game_9() -> Setup {
    transpose(GAME_4)
}

pub fn game_10() -> Setup {
    transpose(GAME_5)
}

pub fn game_11() -> Setup {
    reverse_rows(GAME_1)
}

pub fn game_12() -> Setup {
    reverse_rows(GAME_2)
}

pub fn game_13() -> Setup {
    reverse_rows(GAME_3)
}

pub fn game_14() -> Setup {
    reverse_rows(GAME_4)
}

pub fn game_15() -> Setup {
    reverse_rows(GAME_5)
}

pub trait OccupiedCells {
    fn occupied_cols(&self) -> [u8; 10];
    fn occupied_rows(&self) -> [u8; 10];
}

pub trait Searchable {
    fn find_position(&self, value: u8) -> Option<(usize, usize)>;
}

impl OccupiedCells for Setup {
    fn occupied_cols(&self) -> [u8; 10] {
        occ(&|i, j| self[j][i])
    }
    fn occupied_rows(&self) -> [u8; 10] {
        occ(&|i, j| self[i][j])
    }
}

impl Searchable for Setup {
    fn find_position(&self, value: u8) -> Option<(usize, usize)> {
        for i in MIN_INDEX..=MAX_INDEX {
            for j in MIN_INDEX..=MAX_INDEX {
                if self[i][j] == value {
                    return Some((i, j));
                }
            }
        }
        None
    }
}

fn occ(accessor: &dyn Fn(usize, usize) -> u8) -> [u8; 10] {
    let mut result: [u8; 10] = [0; 10];
    for i in MIN_INDEX..=MAX_INDEX {
        for j in MIN_INDEX..=MAX_INDEX {
            if accessor(i, j) > 0 {
                result[i] += 1;
            };
        }
    }
    result
}

fn transpose(setup: Setup) -> Setup {
    let mut result: Setup = [[0u8; 10]; 10];
    for i in MIN_INDEX..=MAX_INDEX {
        for j in MIN_INDEX..=MAX_INDEX {
            result[j][i] = setup[i][j];
        }
    }
    result
}

fn reverse_rows(setup: Setup) -> Setup {
    let mut result: Setup = [[0u8; 10]; 10];
    for i in MIN_INDEX..=MAX_INDEX {
        for j in MIN_INDEX..=MAX_INDEX {
            result[i][j] = setup[MAX_INDEX - i][j];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use itertools::Itertools;

    #[test]
    fn test_quantity() {
        assert_eq!(build_all().len(), 16);
    }

    #[test]
    fn test_non_equal(){
        assert_eq!(GAME_0, GAME_0);
        assert_ne!(GAME_0, GAME_1);
        for pair in build_all().iter().into_iter().combinations(2) {
            assert_eq!(pair.len(), 2);
            assert_ne!(pair.get(0).unwrap().1.setup, pair.get(1).unwrap().1.setup);
        }
    }

    #[test]
    fn test_transpose() {
        let result: Setup = [
            [00, 00, 08, 09, 10, 00, 11, 12, 00, 00],
            [07, 00, 00, 00, 00, 00, 00, 00, 00, 13],
            [06, 00, 00, 00, 00, 00, 00, 00, 00, 14],
            [05, 00, 00, 04, 00, 00, 00, 00, 00, 00],
            [00, 00, 00, 03, 00, 00, 00, 00, 00, 15],
            [00, 00, 00, 02, 00, 00, 00, 00, 00, 16],
            [00, 00, 00, 01, 00, 00, 00, 00, 00, 00],
            [00, 00, 00, 00, 00, 00, 00, 00, 00, 17],
            [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
            [00, 00, 00, 20, 00, 19, 00, 00, 18, 00],
        ];
        assert_eq!(transpose(GAME_0), result);
    }

    #[test]
    fn test_reverse_rows() {
        let result: Setup = [
            [00, 13, 14, 00, 15, 16, 00, 17, 00, 00],
            [00, 00, 00, 00, 00, 00, 00, 00, 00, 18],
            [12, 00, 00, 00, 00, 00, 00, 00, 00, 00],
            [11, 00, 00, 00, 00, 00, 00, 00, 00, 00],
            [00, 00, 00, 00, 00, 00, 00, 00, 00, 19],
            [10, 00, 00, 00, 00, 00, 00, 00, 00, 00],
            [09, 00, 00, 04, 03, 02, 01, 00, 00, 20],
            [08, 00, 00, 00, 00, 00, 00, 00, 00, 00],
            [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
            [00, 07, 06, 05, 00, 00, 00, 00, 00, 00],
        ];
        assert_eq!(reverse_rows(GAME_0), result);
    }

    #[test_case(GAME_0)]
    #[test_case(GAME_1)]
    #[test_case(GAME_2)]
    #[test_case(GAME_3)]
    #[test_case(GAME_4)]
    #[test_case(GAME_5)]
    #[test_case(game_6())]
    #[test_case(game_7())]
    #[test_case(game_8())]
    #[test_case(game_9())]
    #[test_case(game_10())]
    #[test_case(game_11())]
    #[test_case(game_12())]
    #[test_case(game_13())]
    #[test_case(game_14())]
    #[test_case(game_15())]
    fn test_single_game(setup: Setup) {
        let mut flat: Vec<u8> = Vec::with_capacity(100);
        for row in setup {
            flat.append(&mut row.to_vec());
        }
        assert_eq!(flat.len(), 100);

        let mut non_zero: Vec<u8> = flat.clone().into_iter().filter(|e| *e == 0).collect();
        non_zero.dedup();
        assert_eq!(non_zero, vec![0]);

        let mut non_zero: Vec<u8> = flat.clone().into_iter().filter(|e| *e > 0).collect();
        non_zero.sort();
        let range: Vec<u8> = (1..=20).collect();
        assert_eq!(non_zero, range);

        // corners do not touch
        for i in MIN_INDEX + 1..MAX_INDEX {
            for j in MIN_INDEX + 1..MAX_INDEX {
                if setup[i][j] != 0 {
                    assert_eq!(setup[i - 1][j - 1], 0);
                    assert_eq!(setup[i - 1][j + 1], 0);
                    assert_eq!(setup[i + 1][j - 1], 0);
                    assert_eq!(setup[i + 1][j + 1], 0);
                }
            }
        }
    }
}
