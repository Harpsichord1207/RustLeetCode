use std::collections::HashMap;

pub struct Solution;

struct Cashier {
    n: i32,
    discount: i32,
    product_price: HashMap<i32, i32>,
    c: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let len = products.len();
        let product_price = (0..len)
            .map(|i| (products[i], prices[i]))
            .collect::<HashMap<i32, i32>>();
        Self {
            n, discount, product_price, c: 0
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.c += 1;
        let mut bill = 0;
        for (i, p) in product.into_iter().enumerate() {
            bill += self.product_price.get(&p).unwrap() * amount[i];
        }
        if self.c % self.n == 0 {
            return (100 - self.discount) as f64 / 100f64 * bill as f64
        }
        bill as f64
    }
}

