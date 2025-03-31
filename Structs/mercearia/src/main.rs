use std::io;

#[derive(Debug)]
struct GroceryStore {
    rows: Vec<Row>,
    cash: f32,
}

// Assumes unique product ids
// Assumes a product cannot be in two different places at the same time
// Assumes unique row / shelf / zone names (== ids)
impl GroceryStore {
    fn new() -> GroceryStore {
        GroceryStore {
            rows: vec![],
            cash: 0.0,
        }
    }

    fn check_balance(&self) -> f32 {
       self.cash
    }

    fn add_rows(&mut self, rows: Vec<Row>) {
        self.rows.extend(rows);
    }

    fn add_product(
        &mut self,
        zone_name: &str,
        id: &str,
        name: &str,
        expiration_date: &str,
        price: f32,
        stock: u32,
    ) -> Result<(), &str> {
        if let Some(_) = self.product_position(id) {
            return Err("Product already exists!");
        }

        let product = Product::new(id, name, expiration_date, price, stock);

        if let Some((row_idx, shelf_idx, zone_idx)) = self.zone_position(zone_name) {
            self.rows[row_idx].shelves[shelf_idx].zones[zone_idx]
                .products
                .push(product);
            Ok(())
        } else {
            Err("Zone not found")
        }
    }

    fn remove_product(&mut self, id: &str) -> Result<(), &str> {
        if let Some((row_idx, shelf_idx, zone_idx, product_idx)) = self.product_position(id) {
            self.rows[row_idx].shelves[shelf_idx].zones[zone_idx].products.remove(product_idx);
            Ok(())
        } else {
            Err("Product not found")
        }
    }

    fn restock_product(&mut self, id: &str, units: u32) -> Result<(), &str> {
        if let Some((row_idx, shelf_idx, zone_idx, product_idx)) = self.product_position(id) {
            let product =
                &mut self.rows[row_idx].shelves[shelf_idx].zones[zone_idx].products[product_idx];

            product.stock += units;
            Ok(())
        } else {
            Err("Product not found")
        }
    }

    fn sell_product(&mut self, id: &str, units: u32) -> Result<(), &str> {
        if let Some((row_idx, shelf_idx, zone_idx, product_idx)) = self.product_position(id) {
            let product =
                &mut self.rows[row_idx].shelves[shelf_idx].zones[zone_idx].products[product_idx];

            if product.stock < units {
                Err("Not enough units in stock!")
            } else {
                product.stock -= units;
                self.cash += units as f32 * product.price; // typecast units u32 to f32, in order to multiply to price (f32)
                Ok(())
            }
        } else {
            Err("Product not found")
        }
    }

    fn change_product_name(&mut self, id: &str, new_name: &str) -> Result<(), &str> {
        if let Some((row_idx, shelf_idx, zone_idx, product_idx)) = self.product_position(id) {
            self.rows[row_idx].shelves[shelf_idx].zones[zone_idx].products[product_idx].name =
                new_name.to_string();
            Ok(())
        } else {
            Err("Product not found")
        }
    }

    fn change_product_price(&mut self, id: &str, new_price: f32) -> Result<(), &str> {
        if let Some((row_idx, shelf_idx, zone_idx, product_idx)) = self.product_position(id) {
            self.rows[row_idx].shelves[shelf_idx].zones[zone_idx].products[product_idx].price =
                new_price;
            Ok(())
        } else {
            Err("Product not found")
        }
    }

    fn change_product_position(&mut self, id: &str, new_zone: &str) -> Result<(), &str> {
        if let Some((id_, name, expiration_date, price, stock)) = self.get_product_info(id) {
            self.remove_product(id).map_err(|_| "Failed removing product")?;
            self.add_product(new_zone, id_.as_str(), name.as_str(), expiration_date.as_str(), price, stock)
                .map_err(|error| error)?;
            return Ok(());
        }
        Err("Product not found")
    }

    // auxiliary functions to get indices for row/shelf/zone/product
    // assumes uniqueness

    fn get_product_info(&self, id: &str) -> Option<(String, String, String, f32, u32)> {
        if let Some((row_idx, shelf_idx, zone_idx, product_idx)) = self.product_position(id) {
            let product =
                self.rows[row_idx].shelves[shelf_idx].zones[zone_idx].products[product_idx].clone();
            let Product {
                id: id_,
                name,
                expiration_date,
                price,
                stock,
            } = product;
            Some((id_, name, expiration_date, price, stock))
        } else {
            None
        }
    }

    fn row_position(&self, row_name: &str) -> Option<usize> {
        if let Some(row_idx) = self.rows.iter().position(|row| row.name == row_name) {
            return Some(row_idx);
        }
        None
    }

    fn shelf_position(&self, shelf_name: &str) -> Option<(usize, usize)> {
        for (row_idx, row) in self.rows.iter().enumerate() {
            if let Some(shelf_idx) = row
                .shelves
                .iter()
                .position(|shelf| shelf.name == shelf_name)
            {
                return Some((row_idx, shelf_idx));
            }
        }
        None
    }

    fn zone_position(&self, zone_name: &str) -> Option<(usize, usize, usize)> {
        for (row_idx, row) in self.rows.iter().enumerate() {
            for (shelf_idx, shelf) in row.shelves.iter().enumerate() {
                if let Some(zone_idx) = shelf.zones.iter().position(|zone| zone.name == zone_name) {
                    return Some((row_idx, shelf_idx, zone_idx));
                }
            }
        }
        None
    }

    fn product_position(&self, product_id: &str) -> Option<(usize, usize, usize, usize)> {
        for (row_idx, row) in self.rows.iter().enumerate() {
            for (shelf_idx, shelf) in row.shelves.iter().enumerate() {
                for (zone_idx, zone) in shelf.zones.iter().enumerate() {
                    if let Some(product_idx) = zone
                        .products
                        .iter()
                        .position(|product| product.id == product_id)
                    {
                        return Some((row_idx, shelf_idx, zone_idx, product_idx));
                    }
                }
            }
        }
        None
    }
}
#[derive(Debug)]
struct Row {
    name: String,
    shelves: Vec<Shelf>,
}

impl Row {
    fn new(name: &str, shelves: Vec<Shelf>) -> Row {
        Row {
            name: name.to_string(),
            shelves,
        }
    }
}

#[derive(Debug)]
struct Shelf {
    name: String,
    zones: Vec<Zone>,
}

impl Shelf {
    fn new(name: &str, zones: Vec<Zone>) -> Shelf {
        Shelf {
            name: name.to_string(),
            zones,
        }
    }
}

#[derive(Debug)]
struct Zone {
    name: String,
    products: Vec<Product>,
}

impl Zone {
    fn new(name: &str, products: Vec<Product>) -> Zone {
        Zone {
            name: name.to_string(),
            products,
        }
    }
}

#[derive(Debug, Clone)]
struct Product {
    id: String,
    name: String,
    expiration_date: String,
    price: f32,
    stock: u32,
}

impl Product {
    fn new(id: &str, name: &str, expiration_data: &str, price: f32, stock: u32) -> Product {
        Product {
            id: id.to_string(),
            name: name.to_string(),
            expiration_date: expiration_data.to_string(),
            price,
            stock,
        }
    }
}

fn create_store() -> GroceryStore {
    let mut store = GroceryStore::new();

    let row1 = Row::new(
        "food",
        vec![
            Shelf::new(
                "healthy",
                vec![Zone::new("fruit", vec![]), Zone::new("veggies", vec![])],
            ),
            Shelf::new(
                "snacks",
                vec![Zone::new("chips", vec![]), Zone::new("cookies", vec![])],
            ),
        ],
    );

    let row2 = Row::new(
        "beverages",
        vec![
            Shelf::new(
                "alcohol",
                vec![Zone::new("wine", vec![]), Zone::new("beer", vec![])],
            ),
            Shelf::new(
                "soft",
                vec![Zone::new("juice", vec![]), Zone::new("water", vec![])],
            ),
        ],
    );

    store.add_rows(vec![row1, row2]);

    store
}

fn populate_store(store: &mut GroceryStore) {
    let _ = store.add_product("fruit", "001", "apple", "02-03-2025", 0.99, 50);

    let _ = store.add_product("fruit", "002", "banana", "02-04-2025", 1.49, 25);

    let _ = store.add_product("veggies", "003", "carrot", "03-04-2025", 1.99, 10);

    let _ = store.add_product("veggies", "004", "tomato", "03-05-2025", 1.99, 12);

    let _ = store.add_product("chips", "005", "pringles", "03-05-2026", 2.99, 10);

    let _ = store.add_product("chips", "006", "ruffles", "03-08-2026", 2.49, 8);

    let _ = store.add_product("cookies", "007", "oreo", "03-08-2030", 1.99, 5);

    let _ = store.add_product("cookies", "008", "shortcakes", "03-01-2030", 1.29, 6);

    let _ = store.add_product("wine", "009", "red wine", "03-01-2050", 4.99, 80);

    let _ = store.add_product("wine", "010", "white wine", "08-01-2050", 3.99, 50);

    let _ = store.add_product("beer", "011", "ipa", "08-01-2040", 2.99, 2);

    let _ = store.add_product("beer", "012", "lager", "08-01-2070", 3.49, 3);

    let _ = store.add_product("juice", "013", "orange juice", "08-01-2026", 1.99, 6);

    let _ = store.add_product("juice", "014", "apple juice", "05-01-2026", 2.19, 7);

    let _ = store.add_product("water", "015", "monchique", "05-01-2080", 49.99, 1);

    let _ = store.add_product("water", "016", "fizzle", "01-01-2100", 0.49, 20);
}

// cli stuff
fn option_menu() {
    println!("0: quit");
    println!("1: add new");
    println!("2: remove");
    println!("3: restock");
    println!("4: sell");
    println!("5: change name");
    println!("6: change price");
    println!("7: move");
    println!("8: check balance");
    println!("9: inventory")
}

fn integer_input() -> u32 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    input.parse::<u32>().unwrap() // typecast to u32
}

fn float_input() -> f32 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    input.parse::<f32>().unwrap() // typecast to f32
}

fn string_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn zone_input() -> String {
    println!("Please enter the zone: ");
    string_input()
}

fn product_id_input() -> String {
    println!("Please enter the product id: ");
    string_input()
}

fn product_name_input() -> String {
    println!("Please enter the product name: ");
    string_input()
}

fn product_expiration_date_input() -> String {
    println!("Please enter the product expiration date: ");
    string_input()
}

fn product_price_input() -> f32 {
    println!("Please enter the product price: ");
    float_input()
}
fn product_stock_input() -> u32 {
    println!("Please enter the product stock: ");
    integer_input()
}

fn product_input() -> (String, String, String, f32, u32) {
    let id = product_id_input();
    let name = product_name_input();
    let expiration_date = product_expiration_date_input();
    let price = product_price_input();
    let stock = product_stock_input();

    (id, name, expiration_date, price, stock)
}


fn debug() {
    let mut store = create_store();
    populate_store(&mut store);
    let _ = store.remove_product("001");
    let _ = store.restock_product("005", 1000);
    let _ = store.sell_product("009", 8);
    let _ = store.change_product_name("010", "vinho verde");
    let _ = store.change_product_price("011", 0.01);

    let Some(info) = store.get_product_info("016") else { todo!() };;
    println!("Check some product info: {}, {}, {}", info.0, info.1, info.2);

    match store.change_product_position("016", "juice") {
        Ok(_) => { println!("Successful operation") },
        Err(error) => { println!("Failed to change product position: {}", error) },
    }

    // This throws an error because we don't have enough stock!
    // let res = store.sell_product("beverages", "soft", "water", "015", 2);
    //println!("{:?}", res);

    println!("{:#?}", store);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut store = create_store();
    populate_store(&mut store);

    loop {
        println!("Choose an option:");
        option_menu();
        let option = integer_input();

        match option {
            1 => {  // add
                let zone_name = zone_input();
                let (id, name, expiration_date, price, stock) = product_input();
                store.add_product(zone_name.as_str(), id.as_str(), name.as_str(), expiration_date.as_str(), price, stock)?;
            }
            2 => {  // remove
                let id = product_id_input();
                store.remove_product(id.as_str())?;
            }
            3 => {  // restock
                let id = product_id_input();
                let stock = product_stock_input();
                store.restock_product(id.as_str(), stock)?;
            }
            4 => {  // sell
                let id = product_id_input();
                let stock = product_stock_input();
                store.sell_product(id.as_str(), stock)?;
            }
            5 => {  // change name
                let id = product_id_input();
                let name = product_name_input();
                store.change_product_name(id.as_str(), name.as_str())?;
            },
            6 => {  // change price
                let id = product_id_input();
                let price = product_price_input();
                store.change_product_price(id.as_str(), price)?;
            },
            7 => {  // move
                let id = product_id_input();
                let zone_name = zone_input();
                store.change_product_position(id.as_str(), zone_name.as_str())?;
            },
            8 => {  // check balance
                println!("Available balance: {} $", store.check_balance())
            },
            9 => println!("{:#?}", store),  // inventory
            _ => break,  // quit
        }
    }
    Ok(())
}