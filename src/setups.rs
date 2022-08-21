use im::{hashmap, HashMap};
use uuid::{uuid, Uuid};

pub type Setup = [[u8; 10]; 10];
#[derive(Clone, Copy)]
pub struct SetupFormat{
    pub create_format: CreateFormat,
    pub hint_format: HintFormat,
    pub setup: Setup
}
pub type Setups = HashMap<Uuid, SetupFormat>;

#[derive(Clone, Copy)]
pub enum HintFormat {
    List,
    Nested,
}
#[derive(Clone, Copy)]
pub enum CreateFormat {
    List,
    Nested
}

pub fn build_all() -> Setups {
    hashmap! {
        uuid!("5109c2b1-7c4d-4f56-9be2-f6675c968331") => SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::Nested, setup: GAME_0},
        uuid!("dd8fb490-72c8-485b-aeea-537b9be34e4b") => SetupFormat{create_format: CreateFormat::List,   hint_format: HintFormat::Nested, setup: GAME_1},
        uuid!("37073150-f43d-4609-94ec-dcbeffcb472a") => SetupFormat{create_format: CreateFormat::Nested, hint_format: HintFormat::List,   setup: GAME_2},
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(GAME_0)]
    #[test_case(GAME_1)]
    #[test_case(GAME_2)]
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
