pub fn get_products<const N: usize>(arr: [usize; N]) -> [usize; N] {
    std::array::from_fn(|i| {
        arr.iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &v)| v)
            .product()
    })
}
