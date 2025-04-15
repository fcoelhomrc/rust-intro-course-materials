use std::collections::HashMap;
use std::io;
use std::ops::Not;
use thiserror::Error;

// Merceria 4.0:
// Incremente a merceria ao definir uma trait que defina os comportamentos necessários de um artigo.
// A nossa merceria deve ser capaz de ser utilizada para um tipo de item genérico.
// Devemos manter todas as capacidades anteriores.



#[derive(Debug, Error)]
enum GroceryErrors {
    #[error("Row {0} already exists!")]
    RowAlreadyExists(String),
    #[error("Shelf {1} already exists in row {0}!")]
    ShelfAlreadyExists(String, String),
    #[error("Zone {2} already exists in shelf {1}, row {0}!")]
    ZoneAlreadyExists(String, String, String),
    #[error("Product {3} already exists in zone {2}, shelf {1}, row {0}!")]
    ProductAlreadyExists(String, String, String, Product),
    #[error("Product with ID {0} was not found!")]
    ProductNotFound(String),
    #[error("Row {0} was not found!")]
    RowNotFound(String),
    #[error("Shelf {1} was not found in row {0}!")]
    ShelfNotFound(String, String),
    #[error("Zone {2} was not found in shelf {1}, row {0}!")]
    ZoneNotFound(String, String, String),
    #[error("Overflow when attempting to add {0} to {1}!")]
    U32Overflow(u32, u32),
    #[error("Attempted to sell {1} units of product ID {0}, but found only {2} on stock!")]
    NotEnoughStock(String, u32, u32),
    #[error("Attempted to set price to {0}, but prices should be positive!")]
    InvalidPrice(f32),
    #[error("Invalid combination of arguments! Received row {0:?}, shelf {1:?}, zone {2:?}")]
    InvalidArguments(Option<String>, Option<String>, Option<String>),
}

#[derive(Debug)]
struct GroceryStore {
    rows: HashMap<String, Row>,
    product_map: HashMap<String, (String, String, String)>, // key: ID, value: (row, shelf, zone)
    cash: f32,
}

// Assumes unique product ids
// Assumes a product cannot be in two different places at the same time
// Assumes unique row / shelf / zone names (== ids)
impl GroceryStore {
    fn new() -> Self {
        // Self is an alias to struct name
        Self {
            rows: HashMap::<String, Row>::new(),
            product_map: HashMap::<String, (String, String, String)>::new(),
            cash: 0.0,
        }
    }

    fn add_row(&mut self, row_name: &str) -> Result<(), GroceryErrors> {
        if let Some(_) = self.rows.insert(row_name.to_string(), Row::new()) {
            return Err(GroceryErrors::RowAlreadyExists(row_name.to_string())); // e.g. trying to insert a row that already exists
        } // TODO: return Result Error [✅ Done]
        Ok(())
    }

    fn add_shelf(&mut self, row_name: &str, shelf_name: &str) -> Result<(), GroceryErrors> {
        if let Some(row) = self.rows.get_mut(row_name) {
            if let Some(_) = row.shelves.insert(shelf_name.to_string(), Shelf::new()) {
                return Err(GroceryErrors::ShelfAlreadyExists(
                    row_name.to_string(),
                    shelf_name.to_string(),
                )); // e.g. trying to insert a shelf that already exists in the row
            } // TODO: return Result Error [✅ Done]
        }
        Ok(())
    }

    fn add_zone(
        &mut self,
        row_name: &str,
        shelf_name: &str,
        zone_name: &str,
    ) -> Result<(), GroceryErrors> {
        if let Some(row) = self.rows.get_mut(row_name) {
            if let Some(shelf) = row.shelves.get_mut(shelf_name) {
                if let Some(_) = shelf.zones.insert(zone_name.to_string(), Zone::new()) {
                    return Err(GroceryErrors::ZoneAlreadyExists(
                        row_name.to_string(),
                        shelf_name.to_string(),
                        zone_name.to_string(),
                    )); // e.g. trying to insert a zone that already exists in a shelf in a row
                } // TODO: return Result Error [✅ Done]
            }
        }
        Ok(())
    }

    // consumes product
    fn add_product(
        &mut self,
        row_name: &str,
        shelf_name: &str,
        zone_name: &str,
        product: Product,
    ) -> Result<(), GroceryErrors> {
        // validate position
        let Some(row) = self.rows.get_mut(row_name) else {
            return Err(GroceryErrors::RowNotFound(row_name.to_string()));
        };
        let Some(shelf) = row.shelves.get_mut(shelf_name) else {
            return Err(GroceryErrors::ShelfNotFound(
                row_name.to_string(),
                shelf_name.to_string(),
            ));
        };
        let Some(zone) = shelf.zones.get_mut(zone_name) else {
            return Err(GroceryErrors::ZoneNotFound(
                row_name.to_string(),
                shelf_name.to_string(),
                zone_name.to_string(),
            ));
        };

        // update product_map
        self.product_map.insert(
            product.id.clone(),
            (
                row_name.to_string(),
                shelf_name.to_string(),
                zone_name.to_string(),
            ),
        );

        // add product
        if zone
            .products
            .insert(product.id.clone(), product.clone())
            .is_some()
        {
            return Err(GroceryErrors::ProductAlreadyExists(
                row_name.to_string(),
                shelf_name.to_string(),
                zone_name.to_string(),
                product,
            )); // e.g. trying to insert a product that already exists in a zone in a shelf in a row
        }
        Ok(())
    }

    fn remove_product(&mut self, id: &str) -> Result<Product, GroceryErrors> {
        // find product
        let address = self.find_address_from_id(id);
        let (row_name, shelf_name, zone_name) = match address {
            Some((row, shelf, zone)) => (row, shelf, zone),
            None => return Err(GroceryErrors::ProductNotFound(id.to_string())),
        };

        // remove product
        let Some(row) = self.rows.get_mut(&row_name) else {
            return Err(GroceryErrors::RowNotFound(row_name.to_string()));
        };
        let Some(shelf) = row.shelves.get_mut(&shelf_name) else {
            return Err(GroceryErrors::ShelfNotFound(
                row_name.to_string(),
                shelf_name.to_string(),
            ));
        };
        let Some(zone) = shelf.zones.get_mut(&zone_name) else {
            return Err(GroceryErrors::ZoneNotFound(
                row_name.to_string(),
                shelf_name.to_string(),
                zone_name.to_string(),
            ));
        };

        let product = zone.products.remove(id);
        match product {
            Some(product) => {
                let _ = self.product_map.remove(id);
                Ok(product)
            }
            None => {
                panic!() // We broke a contract: we found the product in the map, but it was not in the correct address
                // TODO: technically we can still recover by repairing the maps, but for this exercise this is out-of-scope
                // Err(GroceryErrors::ProductNotFound(id.to_string())) // FIXME: create more descriptive error
            }
        }
    }

    fn restock_product(&mut self, id: &str, units: u32) -> Result<(), GroceryErrors> {
        // find product
        let Some(product) = self.get_product_mut_from_id(id) else {
            return Err(GroceryErrors::ProductNotFound(id.to_string()));
        };
        // restock product
        product.stock = product
            .stock
            .checked_add(units)
            .ok_or(GroceryErrors::U32Overflow(units, product.stock))?;
        Ok(())
    } // TODO: should return Result [✅ Done]

    fn sell_product(&mut self, id: &str, units: u32) -> Result<(), GroceryErrors> {
        // find product
        let product = self.get_product_mut_from_id(id).unwrap();
        // sell product
        if units > product.stock {
            return Err(GroceryErrors::NotEnoughStock(
                id.to_string(),
                units,
                product.stock,
            ));
        } else {
            product.stock -= units; // no need to check for underflow because of previous clause
            self.cash += product.price * units as f32
        }
        Ok(())
    } // TODO: should return Result [✅ Done]

    fn change_product_name(&mut self, id: &str, new_name: &str) -> Result<(), GroceryErrors> {
        // find product
        let Some(product) = self.get_product_mut_from_id(id) else {
            return Err(GroceryErrors::ProductNotFound(id.to_string()));
        };
        // change name
        product.name = new_name.to_string();
        Ok(())
    } // TODO: should return Result [✅ Done]

    fn change_product_price(&mut self, id: &str, new_price: f32) -> Result<(), GroceryErrors> {
        // check input
        if new_price < 0.0 {
            return Err(GroceryErrors::InvalidPrice(new_price));
        }

        // find product
        let Some(product) = self.get_product_mut_from_id(id) else {
            return Err(GroceryErrors::ProductNotFound(id.to_string()));
        };
        // change name
        product.price = new_price;
        Ok(())
    } // TODO: should return Result [✅ Done]

    fn change_product_position(
        &mut self,
        id: &str,
        row_name: &str,
        shelf_name: &str,
        zone_name: &str,
    ) -> Result<(), GroceryErrors> {
        // remove product (internally updates product map)
        let product = self.get_product_from_id(id);
        if let Some(product) = product {
            self.add_product(row_name, shelf_name, zone_name, product.clone())?;
        };
        let _ = self.remove_product(id)?;
        Ok(())
        // add product (internally updates product map)
    } // TODO: should return Result [✅ Done]

    // methods to find products in the store

    // requires lookups - efficient
    fn get_product_from_id(&self, product_id: &str) -> Option<&Product> {
        let Some((row_name, shelf_name, zone_name)) = self.find_address_from_id(product_id) else {
            return None; // product id not found!
        };

        self.rows
            .get(&row_name)?
            .shelves
            .get(&shelf_name)?
            .zones
            .get(&zone_name)?
            .products
            .get(product_id)
    }

    // requires lookups - efficient
    fn get_product_mut_from_id(&mut self, product_id: &str) -> Option<&mut Product> {
        let Some((row_name, shelf_name, zone_name)) = self.find_address_from_id(product_id) else {
            return None; // product id not found!
        };

        self.rows
            .get_mut(&row_name)?
            .shelves
            .get_mut(&shelf_name)?
            .zones
            .get_mut(&zone_name)?
            .products
            .get_mut(product_id)
    }

    // requires lookups - efficient
    fn find_address_from_id(&self, product_id: &str) -> Option<(String, String, String)> {
        let address = self
            .product_map
            .get(product_id)
            .map(|(row_name, shelf_name, zone_name)| {
                (row_name.clone(), shelf_name.clone(), zone_name.clone())
            });
        address
    }

    // requires lookups - efficient
    fn list_products_at_address(
        &self,
        row_name: Option<&str>,
        shelf_name: Option<&str>,
        zone_name: Option<&str>,
    ) -> Result<(), GroceryErrors> {
        // cool example to have variable number of arguments
        match (row_name, shelf_name, zone_name) {
            // matching a tuple!
            (Some(row_name), Some(shelf_name), Some(zone_name)) => {
                self.list_products_at_zone(row_name, shelf_name, zone_name)
            }
            (Some(row_name), Some(shelf_name), None) => {
                self.list_products_at_shelf(row_name, shelf_name)
            }
            (Some(row_name), None, None) => self.list_products_at_row(row_name),
            (None, None, None) => self.list_products(),
            _ => {
                let (row_name, shelf_name, zone_name) = {
                    (
                        row_name.map(str::to_string),
                        shelf_name.map(str::to_string),
                        zone_name.map(str::to_string),
                    )
                };
                return Err(GroceryErrors::InvalidArguments(
                    row_name, shelf_name, zone_name,
                )); // FIXME: result: Error (incorrect arguments) [✅ Done]
            }
        }
        Ok(())
    }

    fn list_products(&self) {
        self.product_map.iter().for_each(|(id, _)| {
            if let Some(prod) = self.get_product_from_id(id) {
                println!("{}", prod)
            }
        })
    }

    fn list_products_at_row(&self, row_name: &str) {
        self.product_map
            .iter()
            .filter(|(_, address)| address.0 == row_name)
            .for_each(|(id, _)| {
                if let Some(prod) = self.get_product_from_id(id) {
                    println!("{}", prod)
                }
            })
    }
    fn list_products_at_shelf(&self, row_name: &str, shelf_name: &str) {
        self.product_map
            .iter()
            .filter(|(_, address)| address.0 == row_name && address.1 == shelf_name)
            .for_each(|(id, _)| {
                if let Some(prod) = self.get_product_from_id(id) {
                    println!("{}", prod)
                }
            })
    }

    fn list_products_at_zone(&self, row_name: &str, shelf_name: &str, zone_name: &str) {
        self.product_map
            .iter()
            .filter(|(_, address)| {
                address.0 == row_name && address.1 == shelf_name && address.2 == zone_name
            })
            .for_each(|(id, _)| {
                if let Some(prod) = self.get_product_from_id(id) {
                    println!("{}", prod)
                }
            })
    }
}

#[derive(Debug, Clone)]
struct Row {
    shelves: HashMap<String, Shelf>,
}

impl Row {
    fn new() -> Self {
        Self {
            shelves: HashMap::<String, Shelf>::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Shelf {
    zones: HashMap<String, Zone>,
}

impl Shelf {
    fn new() -> Self {
        Self {
            zones: HashMap::<String, Zone>::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Zone {
    products: HashMap<String, Product>,
}

impl Zone {
    fn new() -> Self {
        Self {
            products: HashMap::<String, Product>::new(),
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
    fn new(id: &str, name: &str, expiration_date: &str, price: f32, stock: u32) -> Product {
        Product {
            id: id.to_string(),
            name: name.to_string(),
            expiration_date: expiration_date.to_string(),
            price,
            stock,
        }
    }
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[Product {}] [{}] [Expires at {}] [€{}] [{} units in stock]",
            &self.id, &self.name, &self.expiration_date, &self.price, &self.stock
        )
    }
}

fn build_store() -> Result<GroceryStore, GroceryErrors> {
    let mut store = GroceryStore::new();

    store.add_row("food")?;
    store.add_row("beverages")?;

    store.add_shelf("food", "healthy")?;
    store.add_shelf("food", "snacks")?;

    store.add_shelf("beverages", "alcohol")?;
    store.add_shelf("beverages", "soft_drinks")?;

    store.add_zone("food", "healthy", "fruits")?;
    store.add_zone("food", "healthy", "veggies")?;
    store.add_zone("food", "snacks", "chips")?;
    store.add_zone("food", "snacks", "cookies")?;
    store.add_zone("beverages", "alcohol", "beers")?;
    store.add_zone("beverages", "alcohol", "wines")?;
    store.add_zone("beverages", "soft_drinks", "juices")?;
    store.add_zone("beverages", "soft_drinks", "sodas")?;

    Ok(store)
}

fn populate_store(store: &mut GroceryStore) -> Result<(), GroceryErrors> {
    store.add_product(
        "food",
        "healthy",
        "fruits",
        Product::new("01", "apple", "2025-04-20", 1.20, 50),
    )?;
    store.add_product(
        "food",
        "healthy",
        "veggies",
        Product::new("02", "carrot", "2025-05-10", 0.80, 40),
    )?;
    store.add_product(
        "food",
        "snacks",
        "chips",
        Product::new("03", "pringles", "2026-01-15", 2.50, 30),
    )?;
    store.add_product(
        "food",
        "snacks",
        "cookies",
        Product::new("04", "oreo", "2026-03-05", 3.00, 25),
    )?;
    store.add_product(
        "beverages",
        "alcohol",
        "beers",
        Product::new("05", "superbock", "2026-12-01", 1.50, 60),
    )?;
    store.add_product(
        "beverages",
        "alcohol",
        "wines",
        Product::new("06", "red_wine", "2030-06-01", 10.00, 20),
    )?;
    store.add_product(
        "beverages",
        "soft_drinks",
        "juices",
        Product::new("07", "orange_juice", "2025-07-15", 2.20, 35),
    )?;
    store.add_product(
        "beverages",
        "soft_drinks",
        "sodas",
        Product::new("08", "coke", "2026-02-20", 1.75, 45),
    )?;

    store.add_product(
        "food",
        "healthy",
        "fruits",
        Product::new("09", "banana", "2025-04-15", 1.10, 55),
    )?;
    store.add_product(
        "food",
        "healthy",
        "veggies",
        Product::new("10", "tomato", "2025-04-30", 1.30, 40),
    )?;
    store.add_product(
        "food",
        "snacks",
        "chips",
        Product::new("11", "cheetos", "2026-02-10", 2.80, 28),
    )?;
    store.add_product(
        "food",
        "snacks",
        "cookies",
        Product::new("12", "filipinos", "2026-05-01", 3.20, 22),
    )?;
    store.add_product(
        "beverages",
        "alcohol",
        "beers",
        Product::new("13", "sagres", "2027-01-10", 1.60, 58),
    )?;
    store.add_product(
        "beverages",
        "alcohol",
        "wines",
        Product::new("14", "white_wine", "2031-08-15", 12.00, 18),
    )?;
    store.add_product(
        "beverages",
        "soft_drinks",
        "juices",
        Product::new("15", "apple_juice", "2025-09-10", 2.40, 33),
    )?;
    store.add_product(
        "beverages",
        "soft_drinks",
        "sodas",
        Product::new("16", "pepsi", "2026-03-25", 1.80, 42),
    )?;
    Ok(())
}

// CLI STUFF
#[derive(Debug, Error)]
enum CLIErrors {
    #[error("Invalid input: {0}")]
    IOError(io::Error),
    #[error("Failed to parse to u32: {0}")]
    ParseIntError(std::num::ParseIntError),
    #[error("Failed to parse to f32: {0}")]
    ParseFloatError(std::num::ParseFloatError),
}

fn integer_input(display: Option<&str>) -> Result<u32, CLIErrors> {
    if let Some(display) = display {
        println!("{}", display)
    };

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    match input.parse::<u32>() {
        Ok(i) => Ok(i),
        Err(e) => Err(CLIErrors::ParseIntError(e)),
    }
}

fn float_input(display: Option<&str>) -> Result<f32, CLIErrors> {
    if let Some(display) = display {
        println!("{}", display)
    };

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    match input.parse::<f32>() {
        Ok(i) => Ok(i),
        Err(e) => Err(CLIErrors::ParseFloatError(e)),
    }
}

fn string_input(display: Option<&str>) -> Result<String, CLIErrors> {
    if let Some(display) = display {
        println!("{}", display)
    };

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(e) => Err(CLIErrors::IOError(e)),
    }
}

fn address_input() -> Result<(String, String, String), CLIErrors> {
    let row_name = string_input(Some("Enter row: "))?;
    let shelf_name = string_input(Some("Enter shelf: "))?;
    let zone_name = string_input(Some("Enter zone: "))?;

    Ok((row_name, shelf_name, zone_name))
}

fn partial_address_input() -> Result<(Option<String>, Option<String>, Option<String>), CLIErrors> {
    println!("Hint: passing an empty string lists all products in the current partial address");
    let row_name = string_input(Some("Enter row: "))?;
    let row_name = row_name.is_empty().not().then(|| row_name);
    if row_name.is_none() {
        return Ok((None, None, None)); // list all products
    }

    let shelf_name = string_input(Some("Enter shelf: "))?;
    let shelf_name = shelf_name.is_empty().not().then(|| shelf_name);
    if shelf_name.is_none() {
        return Ok((row_name, None, None)); // list all products in given row
    }

    let zone_name = string_input(Some("Enter zone: "))?;
    let zone_name = zone_name.is_empty().not().then(|| zone_name);
    if zone_name.is_none() {
        return Ok((row_name, shelf_name, None)); // list all products in given row/shelf
    }

    Ok((row_name, shelf_name, zone_name)) // list all products in given row/shelf/zone
}

fn product_id_input() -> Result<String, CLIErrors> {
    string_input(Some("Enter id: "))
}

fn product_input() -> Result<Product, CLIErrors> {
    let id = product_id_input()?;
    let name = string_input(Some("Enter name: "))?;
    let expiration_date = string_input(Some("Enter expiration_date: "))?;
    let price = float_input(Some("Enter price: "))?;
    let stock = integer_input(Some("Enter stock: "))?;

    let product = Product::new(
        id.as_str(),
        name.as_str(),
        expiration_date.as_str(),
        price,
        stock,
    );
    Ok(product)
}

fn address_hint() {
    // FIXME: hardcoded because I am lazy and these are invariants anyway
    println!("Store map: ");
    println!(
        "food
├── healthy
│   ├── fruits
│   └── veggies
└── snacks
    ├── chips
    └── cookies
beverages
├── alcohol
│   ├── beers
│   └── wines
└── soft_drinks
    ├── juices
    └── sodas"
    );
    println!("---------------------------")
}

fn welcome_msg() {
    println!("Welcome to Sweet Drop!");
    println!("It feels good to pay so little!");
}

fn option_menu() {
    println!("0: quit");
    println!("1: add product");
    println!("2: remove product");
    println!("3: restock product");
    println!("4: sell product");
    println!("5: change product name");
    println!("6: change product price");
    println!("7: change product location");
    println!("8: inspect store balance");
    println!("9: inspect store inventory");
}

fn invalid_input_warning() {
    println!("Invalid input!");
}

// main
fn main() {
    let mut store = build_store().expect("Hardcoded, should not fail");
    populate_store(&mut store).expect("Hardcoded, should not fail");

    welcome_msg();

    loop {
        option_menu();
        let Ok(option) = integer_input(Some("Enter option: ")) else {
            invalid_input_warning();
            continue;
        };
        match option {
            0 => break,
            1 => {
                let Ok(product) = product_input() else {
                    invalid_input_warning();
                    continue;
                };
                let Ok((row_name, shelf_name, zone_name)) = address_input() else {
                    invalid_input_warning();
                    continue;
                };
                match store.add_product(
                    row_name.as_str(),
                    shelf_name.as_str(),
                    zone_name.as_str(),
                    product,
                ) {
                    Ok(_) => continue,
                    Err(error) => println!("{}", error),
                }
            }
            2 => {
                let Ok(id) = product_id_input() else {
                    invalid_input_warning();
                    continue;
                };
                match store.remove_product(id.as_str()) {
                    Ok(_) => continue,
                    Err(error) => println!("{}", error),
                }
            }
            3 => {
                let Ok(id) = product_id_input() else {
                    invalid_input_warning();
                    continue;
                };
                let units = match integer_input(Some("Enter units: ")) {
                    Ok(i) => i,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                match store.restock_product(id.as_str(), units) {
                    Ok(_) => continue,
                    Err(error) => println!("{}", error),
                }
            }
            4 => {
                let Ok(id) = product_id_input() else {
                    invalid_input_warning();
                    continue;
                };
                let units = match integer_input(Some("Enter units: ")) {
                    Ok(i) => i,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                match store.sell_product(id.as_str(), units) {
                    Ok(_) => continue,
                    Err(error) => println!("{}", error),
                }
            }
            5 => {
                let Ok(id) = product_id_input() else {
                    invalid_input_warning();
                    continue;
                };
                let Ok(name) = string_input(Some("Enter name: ")) else {
                    invalid_input_warning();
                    continue;
                };
                match store.change_product_name(id.as_str(), name.as_str()) {
                    Ok(_) => continue,
                    Err(error) => println!("{}", error),
                }
            }
            6 => {
                let Ok(id) = product_id_input() else {
                    invalid_input_warning();
                    continue;
                };
                let price = match float_input(Some("Enter price: ")) {
                    Ok(i) => i,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                match store.change_product_price(id.as_str(), price) {
                    Ok(_) => continue,
                    Err(error) => println!("{}", error),
                }
            }
            7 => {
                address_hint();
                let Ok(id) = product_id_input() else {
                    invalid_input_warning();
                    continue;
                };
                let (row_name, shelf_name, zone_name) = match address_input() {
                    Ok((row, shelf, zone)) => (row, shelf, zone),
                    Err(error) => {
                        println!("{}", error);
                        continue;
                    }
                };

                match store.change_product_position(
                    id.as_str(),
                    row_name.as_str(),
                    shelf_name.as_str(),
                    zone_name.as_str(),
                ) {
                    Ok(_) => continue,
                    Err(error) => println!("{}", error),
                };
            }
            8 => println!("Balance: €{}", &store.cash),
            9 => {
                address_hint();
                let (row_name, shelf_name, zone_name) = match partial_address_input() {
                    Ok((row, shelf, zone)) => (row, shelf, zone),
                    Err(error) => {
                        println!("{}", error);
                        continue;
                    }
                };
                let (row_name, shelf_name, zone_name) = {
                    (
                        row_name.as_deref(),
                        shelf_name.as_deref(),
                        zone_name.as_deref(),
                    )
                };
                match store.list_products_at_address(row_name, shelf_name, zone_name) {
                    Ok(_) => continue,
                    Err(error) => println!("{}", error),
                }
            }
            _ => println!("Error: invalid option"),
        }
    }
}
