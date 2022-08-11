type Template = [[u8; 10]; 10];

const GAME_0: Template = [
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

const GAME_1: Template = [
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

const GAME_2: Template = [
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_1() {
        //test_single_game(GAME_0);
        test_single_game(GAME_1);
        test_single_game(GAME_2);
    }

    fn test_single_game(template: Template) {
        let mut flat: Vec<u8> = Vec::with_capacity(100);
        for row in template {
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
