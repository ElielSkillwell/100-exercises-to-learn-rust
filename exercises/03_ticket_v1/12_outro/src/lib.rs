// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}


impl Order {
    pub fn new(new_product_name: String, new_quantity: u32, new_unit_price: u32) -> Order {
        Order::check_name_is_valid(&new_product_name);
        Order::check_quantity_is_valid(&new_quantity);
        Order::check_unit_price_is_valid(&new_unit_price);

        Order {
            product_name: new_product_name,
            quantity: new_quantity,
            unit_price: new_unit_price
        }
    }

    fn check_name_is_valid(name: &String) {
        if name.is_empty() || name.len() > 300 {
            panic!("Name does not fulfill restrictions.")
        }
    }

    fn check_quantity_is_valid(quantity: &u32) {
        if *quantity <= 0_u32 {
            panic!("Quantity is zero or is less than")
        }
    }

    fn check_unit_price_is_valid(unit_price: &u32) {
        if *unit_price <= 0_u32 {
            panic!("unit_price is less than or is zero.")
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        &self.quantity * &self.unit_price
    }

    pub fn set_product_name(&mut self, name: String) {
        Order::check_name_is_valid(&name);
        self.product_name = name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        Order::check_quantity_is_valid(&quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, price: u32) {
        Order::check_unit_price_is_valid(&price);
        self.unit_price = price;
    }

}