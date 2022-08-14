use im::{hashmap, HashMap};
use uuid::{uuid, Uuid};

pub type Setup = [[u8; 10]; 10];
pub type Setups = HashMap<Uuid, Setup>;

pub fn build_all() -> Setups {
    hashmap! {
        uuid!("5109c2b1-7c4d-4f56-9be2-f6675c968331") => GAME_0,
        uuid!("dd8fb490-72c8-485b-aeea-537b9be34e4b") => GAME_1,
        uuid!("37073150-f43d-4609-94ec-dcbeffcb472a") => GAME_2,
    }
}

pub const GAME_0: Setup = [
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
    [00, 00, 00, 00, 00, 00, 00, 00, 00, 00],
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

impl OccupiedCells for Setup {
    fn occupied_cols(&self) -> [u8; 10] {
        occ(&|i, j| self[i][j])
    }
    fn occupied_rows(&self) -> [u8; 10] {
        occ(&|i, j| self[j][i])
    }
}

fn occ(accessor: &dyn Fn(usize, usize) -> u8) -> [u8; 10] {
    let mut result: [u8; 10] = [0; 10];
    for i in 0..=9 {
        for j in 0..=9 {
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

    #[test]
    pub fn test_all_templates() {
        for t in build_all() {
            test_single_game(t.1);    
        }
    }

    fn test_single_game(setup: Setup) {
        let mut flat: Vec<u8> = Vec::with_capacity(100);
        for row in setup {
            flat.append(&mut row.to_vec());
        }
        assert_eq!(100, flat.len());

        let mut non_zero: Vec<u8> = flat.clone().into_iter().filter(|e| *e == 0).collect();
        non_zero.dedup();
        assert_eq!(vec![0], non_zero);

        let mut non_zero: Vec<u8> = flat.clone().into_iter().filter(|e| *e > 0).collect();
        non_zero.sort();
        let range: Vec<u8> = (1..=20).collect();
        assert_eq!(range, non_zero)
    }
}
