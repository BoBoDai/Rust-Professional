const CASHES: [u32; 8] = [100, 50, 30, 20, 10, 5, 2, 1];
pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut amount = amount;
    let mut result = 0;
    for c in CASHES {
        let count = amount / c;
        result += count;
        amount -= count * c;
    }
    result
}
