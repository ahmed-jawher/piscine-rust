#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item<'a>(pub &'a str, pub f64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Store<'a, const N: usize> {
    pub products: [Item<'a>; N],
}

impl<'a, const N: usize> Store<'a, N> {
    pub fn new(products: [Item<'a>; N]) -> Self {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart<'a> {
    pub items: Vec<Item<'a>>,
}

impl<'a> Cart<'a> {
    pub fn new() -> Self {
        Cart { items: Vec::new() }
    }

    pub fn insert_item_by_name<const N: usize>(
        &mut self,
        s: &Store<'a, N>,
        name: &str,
    ) -> Result<(), ()> {
        let item = s.products.iter().find(|item| item.0 == name).ok_or(())?;
        self.items.push(*item);
        Ok(())
    }

    pub fn generate_receipt(&self) -> Vec<f64> {
        let mut prices: Vec<f64> = self.items.iter().map(|item| item.1).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let total: f64 = prices.iter().sum();
        let free: f64 = prices.iter().take(prices.len() / 3).sum();
        let ratio = if total == 0.0 { 1.0 } else { (total - free) / total };

        prices
            .iter()
            .map(|p| (p * ratio * 100.0).round() / 100.0)
            .collect()
    }
}
