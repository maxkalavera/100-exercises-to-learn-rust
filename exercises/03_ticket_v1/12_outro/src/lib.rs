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
    unit_price: u32,
}


// Validators or enforcers
impl Order {
    fn enforce_product_name (
        product_name: String
    ) -> String {
        if product_name == "" || product_name.len() > 300 {
            panic!("The product name can't be empty and it can't be longer than 300 bytes.")
        }
        product_name
    }

    fn enforce_quantity (
        quantity: u32
    ) -> u32 {
        if quantity <= 0 {
            panic!("The quantity must be strictly greater than zero.")
        }
        quantity
    }

    fn enforce_unit_price (
        unit_price: u32
    ) -> u32 {
        if unit_price <= 0 {
            panic!("The unit price is in cents and must be strictly greater than zero.")
        }
        unit_price
    }
}

impl Order {

    pub fn new (
        product_name: String,
        quantity: u32,
        unit_price: u32,
    ) -> Order {
        let product_name = Order::enforce_product_name(product_name);
        let quantity = Order::enforce_quantity(quantity);
        let unit_price = Order::enforce_unit_price(unit_price);

        Order {
            product_name, 
            quantity, 
            unit_price
        }
    }

    pub fn total (self: &Order) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn product_name (self: &Order) -> &str { 
        &self.product_name 
    }

    pub fn quantity (self: &Order) -> &u32 { 
        &self.quantity 
    }

    pub fn unit_price (self: &Order) -> &u32 { 
        &self.unit_price 
    }

    pub fn set_product_name (
        self: &mut Order , 
        product_name: String
    ) {
        self.product_name = product_name;
    }

    pub fn set_quantity (
        self: &mut Order, 
        quantity: u32
    ) {
        self.quantity = quantity;
    }

    pub fn set_unit_price (
        self: &mut Order, 
        unit_price: u32
    ) {
        self.unit_price = unit_price;
    }

}