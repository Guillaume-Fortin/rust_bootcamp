// Make the following code compile.

struct ShopItem {
    name: String,
    quantity: u32,
    in_stock: bool,
}

fn main() {
    let mut item = ShopItem {
        name: String::from("Socks"),
        quantity: 200,
        in_stock: true,
    };
    // 50 pairs of socks were sold
    item.quantity -= 50;
    if item.quantity == 0 {
        item.in_stock = false;
    }
    println!("{} is in stock: {}", item.name, item.in_stock);
}
