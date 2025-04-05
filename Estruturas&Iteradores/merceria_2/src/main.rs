use std::collections::HashMap;
use std::io;
use std::ops::Not;

///
/// Mercearia 2.0
///
/// Para implementar esta iteração do exercício, deve copiar a versão anterior Mercearia 1.0
/// E fazer todas as alterações pedidas pelo enunciado.
///
/// Devem manter ambas as versões do exercício.

/// Merceria 2.0: Augmente a Merceria para podermos encontrar eficientemente
/// os produtos numa fileira, prateleira e zona.
/// Devemos também ser capazes de encontrar eficientemente um produto e a sua posição.

#[derive(Debug)]
struct GroceryStore {
    rows: HashMap<String, Row>,
    product_map: HashMap<String, (String, String, String)>,
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

    fn add_row(&mut self, row_name: &str) {
        if let Some(_) = self.rows.insert(row_name.to_string(), Row::new()) {
            todo!() // e.g. trying to insert a row that already exists
        }
    }

    fn add_shelf(&mut self, row_name: &str, shelf_name: &str) {
        if let Some(row) = self.rows.get_mut(row_name) {
            if let Some(_) = row.shelves.insert(shelf_name.to_string(), Shelf::new()) {
                todo!() // e.g. trying to insert a shelf that already exists in the row
            }
        }
    }

    fn add_zone(&mut self, row_name: &str, shelf_name: &str, zone_name: &str) {
        if let Some(row) = self.rows.get_mut(row_name) {
            if let Some(shelf) = row.shelves.get_mut(shelf_name) {
                if let Some(_) = shelf.zones.insert(zone_name.to_string(), Zone::new()) {
                    todo!() // e.g. trying to insert a zone that already exists in a shelf in a row
                }
            }
        }
    }

    // consumes product
    fn add_product(&mut self, row_name: &str, shelf_name: &str, zone_name: &str, product: Product) {
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
        if let Some(row) = self.rows.get_mut(row_name) {
            if let Some(shelf) = row.shelves.get_mut(shelf_name) {
                if let Some(zone) = shelf.zones.get_mut(zone_name) {
                    if let Some(_) = zone.products.insert(product.id.clone(), product) {
                        todo!() // e.g. trying to insert a product that already exists in a zone in a shelf in a row
                    }
                }
            }
        }
    }

    fn remove_product(&mut self, id: &str) -> Option<Product> {
        // find product
        let Some((row_name, shelf_name, zone_name)) = self.find_address_from_id(id) else {
            todo!()
        };

        // remove product
        let product = self
            .rows
            .get_mut(&row_name)?
            .shelves
            .get_mut(&shelf_name)?
            .zones
            .get_mut(&zone_name)?
            .products
            .remove(id);

        // update product_map
        let _ = self.product_map.remove(id);
        product
    }

    fn restock_product(&mut self, id: &str, units: u32) {
        // find product
        let product = self.get_product_mut_from_id(id).unwrap();
        // restock product
        product.stock += units;
    }

    fn sell_product(&mut self, id: &str, units: u32) {
        // find product
        let product = self.get_product_mut_from_id(id).unwrap();
        // sell product
        if units > product.stock {
            println!("Not enough units to sell! Ignoring order..."); //FIXME: do proper error handling
            ()
        } else {
            product.stock -= units;
            self.cash += product.price * units as f32
        }
    }

    fn change_product_name(&mut self, id: &str, new_name: &str) {
        // find product
        let product = self.get_product_mut_from_id(id).unwrap();
        // change name
        product.name = new_name.to_string();
    }

    fn change_product_price(&mut self, id: &str, new_price: f32) {
        // find product
        let product = self.get_product_mut_from_id(id).unwrap();
        // change name
        product.price = new_price;
    }

    fn change_product_position(
        &mut self,
        id: &str,
        row_name: &str,
        shelf_name: &str,
        zone_name: &str,
    ) {
        // remove product (internally updates product map)
        let product = self.remove_product(id).unwrap();
        // add product (internally updates product map)
        self.add_product(row_name, shelf_name, zone_name, product);
    }

    // methods to find products in the store

    // requires lookups - efficient
    fn get_product_from_id(&self, product_id: &str) -> Option<&Product> {
        let Some((row_name, shelf_name, zone_name)) = self.find_address_from_id(product_id) else {
            todo!() // product id not found!
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
            todo!() // product id not found!
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
    ) {
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
                todo!()
            }
        }
    }

    fn list_products(&self) {
        self.product_map
            .iter()
            .for_each(|(id, _)| println!("{:#?}", self.get_product_from_id(id).unwrap()))
    }

    fn list_products_at_row(&self, row_name: &str) {
        self.product_map
            .iter()
            .filter(|(id, address)| address.0 == row_name)
            .for_each(|(id, _)| println!("{:#?}", self.get_product_from_id(id).unwrap()));
    }
    fn list_products_at_shelf(&self, row_name: &str, shelf_name: &str) {
        self.product_map
            .iter()
            .filter(|(id, address)| address.0 == row_name && address.1 == shelf_name)
            .for_each(|(id, _)| println!("{:#?}", self.get_product_from_id(id).unwrap()));
    }

    fn list_products_at_zone(&self, row_name: &str, shelf_name: &str, zone_name: &str) {
        self.product_map
            .iter()
            .filter(|(id, address)| {
                address.0 == row_name && address.1 == shelf_name && address.2 == zone_name
            })
            .for_each(|(id, _)| println!("{:#?}", self.get_product_from_id(id).unwrap()));
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

fn build_store() -> GroceryStore {
    let mut store = GroceryStore::new();

    store.add_row("food");
    store.add_row("beverages");

    store.add_shelf("food", "healthy");
    store.add_shelf("food", "snacks");

    store.add_shelf("beverages", "alcohol");
    store.add_shelf("beverages", "soft_drinks");

    store.add_zone("food", "healthy", "fruits");
    store.add_zone("food", "healthy", "veggies");
    store.add_zone("food", "snacks", "chips");
    store.add_zone("food", "snacks", "cookies");
    store.add_zone("beverages", "alcohol", "beers");
    store.add_zone("beverages", "alcohol", "wines");
    store.add_zone("beverages", "soft_drinks", "juices");
    store.add_zone("beverages", "soft_drinks", "sodas");

    store
}

fn populate_store(store: &mut GroceryStore) {
    store.add_product(
        "food",
        "healthy",
        "fruits",
        Product::new("01", "apple", "2025-04-20", 1.20, 50),
    );
    store.add_product(
        "food",
        "healthy",
        "veggies",
        Product::new("02", "carrot", "2025-05-10", 0.80, 40),
    );
    store.add_product(
        "food",
        "snacks",
        "chips",
        Product::new("03", "pringles", "2026-01-15", 2.50, 30),
    );
    store.add_product(
        "food",
        "snacks",
        "cookies",
        Product::new("04", "oreo", "2026-03-05", 3.00, 25),
    );
    store.add_product(
        "beverages",
        "alcohol",
        "beers",
        Product::new("05", "superbock", "2026-12-01", 1.50, 60),
    );
    store.add_product(
        "beverages",
        "alcohol",
        "wines",
        Product::new("06", "red_wine", "2030-06-01", 10.00, 20),
    );
    store.add_product(
        "beverages",
        "soft_drinks",
        "juices",
        Product::new("07", "orange_juice", "2025-07-15", 2.20, 35),
    );
    store.add_product(
        "beverages",
        "soft_drinks",
        "sodas",
        Product::new("08", "coke", "2026-02-20", 1.75, 45),
    );

    store.add_product(
        "food",
        "healthy",
        "fruits",
        Product::new("09", "banana", "2025-04-15", 1.10, 55),
    );
    store.add_product(
        "food",
        "healthy",
        "veggies",
        Product::new("10", "tomato", "2025-04-30", 1.30, 40),
    );
    store.add_product(
        "food",
        "snacks",
        "chips",
        Product::new("11", "cheetos", "2026-02-10", 2.80, 28),
    );
    store.add_product(
        "food",
        "snacks",
        "cookies",
        Product::new("12", "filipinos", "2026-05-01", 3.20, 22),
    );
    store.add_product(
        "beverages",
        "alcohol",
        "beers",
        Product::new("13", "sagres", "2027-01-10", 1.60, 58),
    );
    store.add_product(
        "beverages",
        "alcohol",
        "wines",
        Product::new("14", "white_wine", "2031-08-15", 12.00, 18),
    );
    store.add_product(
        "beverages",
        "soft_drinks",
        "juices",
        Product::new("15", "apple_juice", "2025-09-10", 2.40, 33),
    );
    store.add_product(
        "beverages",
        "soft_drinks",
        "sodas",
        Product::new("16", "pepsi", "2026-03-25", 1.80, 42),
    );
}


// CLI STUFF
fn integer_input(display: Option<&str>) -> u32 {
    if let Some(display) = display {
        println!("{}", display)
    }

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    input.parse::<u32>().unwrap() // typecast to u32
}

fn float_input(display: Option<&str>) -> f32 {
    if let Some(display) = display {
        println!("{}", display)
    }

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    input.parse::<f32>().unwrap() // typecast to f32
}

fn string_input(display: Option<&str>) -> String {
    if let Some(display) = display {
        println!("{}", display)
    }

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn address_input() -> (String, String, String) {
    let row_name = string_input(Some("Enter row: "));
    let shelf_name = string_input(Some("Enter shelf: "));
    let zone_name = string_input(Some("Enter zone: "));

    (row_name, shelf_name, zone_name)
}

fn partial_address_input() -> (Option<String>, Option<String>, Option<String>) {
    println!("Hint: passing an empty string lists all products in the current partial address");
    let row_name = string_input(Some("Enter row: "));
    let row_name = row_name.is_empty().not().then(|| row_name);
    if row_name.is_none() {
        return (None, None, None);   // list all products
    }

    let shelf_name = string_input(Some("Enter shelf: "));
    let shelf_name = shelf_name.is_empty().not().then(|| shelf_name);
    if shelf_name.is_none() {
        return (row_name, None, None);  // list all products in given row
    }

    let zone_name = string_input(Some("Enter zone: "));
    let zone_name = zone_name.is_empty().not().then(|| zone_name);
    if zone_name.is_none() {
        return (row_name, shelf_name, None);  // list all products in given row/shelf
    }

    (row_name, shelf_name, zone_name)  // list all products in given row/shelf/zone
}


fn product_id_input() -> String {
    string_input(Some("Enter id: "))
}

fn product_input() -> Product {
    let id = product_id_input();
    let name = string_input(Some("Enter name: "));
    let expiration_date = string_input(Some("Enter expiration_date: "));
    let price = float_input(Some("Enter price: "));
    let stock = integer_input(Some("Enter stock: "));

    Product::new(id.as_str(), name.as_str(), expiration_date.as_str(), price, stock)
}

fn address_hint() {
    // FIXME: hardcoded because I am lazy and these are invariants anyway
    println!("Store map: ");
    println!("food
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

// main
fn main() {
    let mut store = build_store();
    populate_store(&mut store);

    welcome_msg();

    loop {
        option_menu();
        let option = integer_input(Some("Enter option: "));
        match option {
            0 => break,
            1 => {
                let product = product_input();
                let (row_name, shelf_name, zone_name) = address_input();
                store.add_product(row_name.as_str(), shelf_name.as_str(), zone_name.as_str(), product);
            },
            2 => {
                let id = product_id_input();
                store.remove_product(id.as_str());
            },
            3 => {
                let id = product_id_input();
                let units = integer_input(Some("Enter units: "));
                store.restock_product(id.as_str(), units);
            },
            4 => {
                let id = product_id_input();
                let units = integer_input(Some("Enter units: "));
                store.sell_product(id.as_str(), units);
            },
            5 => {
                let id = product_id_input();
                let name = string_input(Some("Enter name: "));
                store.change_product_name(id.as_str(), name.as_str());
            },
            6 => {
                let id = product_id_input();
                let price = float_input(Some("Enter price: "));
                store.change_product_price(id.as_str(), price);
            },
            7 => {
                let id = product_id_input();
                let (row_name, shelf_name, zone_name) = address_input();
                store.change_product_position(id.as_str(), row_name.as_str(), shelf_name.as_str(), zone_name.as_str());
            },
            8 => println!("Balance: €{}", &store.cash),
            9 => {
                address_hint();
                let (row_name, shelf_name, zone_name) = partial_address_input();
                let (row_name, shelf_name, zone_name) = {
                    (row_name.as_deref(), shelf_name.as_deref(), zone_name.as_deref())
                };
                store.list_products_at_address(row_name, shelf_name, zone_name);
            },
            _ => println!("Error: invalid option"),
        }
    }
}



fn debug() {
    let mut store = build_store();
    populate_store(&mut store);
    println!("{:#?}", store);

    println!("########## ADD ##########");
    store.add_product(
        "beverages",
        "soft_drinks",
        "sodas",
        Product::new("17", "fanta", "2026-03-25", 1.90, 30),
    );
    println!("{:#?}", store);

    println!("######### RESTOCK #########");
    store.restock_product("17", 100);
    println!("{:#?}", store);

    println!("######## SELL ###########");
    store.sell_product("17", 100);
    println!("{:#?}", store);

    println!("######## CHANGE NAME ############");
    store.change_product_name("17", "porto_wine");
    println!("{:#?}", store);

    println!("########## CHANGE PRICE  #########");
    store.change_product_price("17", 16.50);
    println!("{:#?}", store);

    println!("######### CHANGE POSITION #########");
    store.change_product_position("17", "beverages", "alcohol", "wines");
    println!("{:#?}", store);

    println!("######### LIST AT ADDRESS (ROW, _, _) #########");
    store.list_products_at_address(Some("food"), None, None);

    println!("######### LIST AT ADDRESS (ROW, SHELF, _) #########");
    store.list_products_at_address(Some("food"), Some("healthy"), None);

    println!("######### LIST AT ADDRESS (ROW, SHELF, ZONE) #########");
    store.list_products_at_address(Some("food"), Some("healthy"), Some("fruits"));

    println!("######## REMOVE #########");
    store.remove_product("17");
    println!("{:#?}", store);
}
