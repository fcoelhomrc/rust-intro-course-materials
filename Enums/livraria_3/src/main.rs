use std::collections::{HashMap, HashSet};

// Livraria 3.0
// Extender a livraria anterior para poder ter livros, audio books, estátuas e quadros.
// Deve manter as mesmas capacidades.
// Deve também ter o máximo de elementos comuns em zonas partilhadas,
// utilizando composição para partilhar o máximo de código possivel
// (por exemplo, todos os elementos têm título e autor,
// mas apenas os audio books têm durações,
// apenas as estátuas e quadros têm dimensões físicas).

// TODO! Do we need to have reverse maps for ALL variant-specific properties?
// TODO! Where can we share behavior?
// TODO! Implement CLI...

use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ArtifactKindName {
    Book,
    AudioBook,
    Statue,
    Painting,
}

#[derive(Debug, Clone)]
enum ArtifactKind {
    Book {
        pages: u32,
        isbn: String,
    },
    AudioBook {
        duration_minutes: f32,
        narrator: String,
    },
    Statue {
        dimensions_cm: (f32, f32),
        height_cm: f32,
        weight_kg: f32,
        material: String,
    },
    Painting {
        dimensions_cm: (f32, f32),
        style: String,
        medium: String,
    },
}

impl ArtifactKind {
    fn name(&self) -> ArtifactKindName {
        // NEW: two enums - one to hold variant-specific data, other for comparing/hashing on variants
        match self {
            ArtifactKind::Book { .. } => ArtifactKindName::Book,
            ArtifactKind::AudioBook { .. } => ArtifactKindName::AudioBook,
            ArtifactKind::Statue { .. } => ArtifactKindName::Statue,
            ArtifactKind::Painting { .. } => ArtifactKindName::Painting,
        }
    }
}

#[derive(Debug, Clone)]
struct Artifact {
    id: u32,
    units: u32,
    title: String,
    author: String,
    keywords: Vec<String>, // TODO: upgrade to HashSet for querying
    kind: ArtifactKind,
}

impl Artifact {
    fn new(
        id: u32,
        units: Option<u32>,
        title: &str,
        author: &str,
        keywords: Vec<&str>,
        kind: ArtifactKind,
    ) -> Self {
        Self {
            id,
            units: units.unwrap_or(1),
            title: title.to_string(),
            author: author.to_string(),
            keywords: keywords.into_iter().map(String::from).collect(),
            kind,
        }
    }
}

#[derive(Debug)]
struct Library {
    artifacts: HashMap<u32, Artifact>,
    artifacts_by_title: HashMap<String, HashSet<u32>>,
    artifacts_by_author: HashMap<String, HashSet<u32>>,
    artifacts_by_keyword: HashMap<String, HashSet<u32>>,
    // TODO! Implement variant-specific reverse maps to allow querying
    // TODO! Implement Kind reverse map to query by Artifact variant
    artifacts_by_kind: HashMap<ArtifactKindName, HashSet<u32>>,
}

impl Library {
    // FIXME: fill up the HashMaps with the artifact Vec
    fn new(artifacts: Vec<Artifact>) -> Self {
        let mut instance = Self {
            artifacts: HashMap::<u32, Artifact>::new(),
            artifacts_by_title: HashMap::<String, HashSet<u32>>::new(),
            artifacts_by_author: HashMap::<String, HashSet<u32>>::new(),
            artifacts_by_keyword: HashMap::<String, HashSet<u32>>::new(),
            artifacts_by_kind: HashMap::<ArtifactKindName, HashSet<u32>>::new(),
        };
        for artifact in artifacts {
            instance.add_artifact(artifact); // updates reverse maps automatically
        }
        instance
    }

    // AUXILIARY METHODS
    fn list_ids(&self) -> Vec<u32> {
        self.artifacts.keys().copied().collect::<Vec<u32>>()
    }

    fn has_artifact(&self, id: u32) -> bool {
        self.artifacts.contains_key(&id)
    }

    fn has_available_artifact(&self, id: u32, units: u32) -> bool {
        if self.has_artifact(id) {
            self.artifacts.get(&id).unwrap().units >= units // FIXME: handle Option properly
        } else {
            false
        }
    }

    fn can_lend(&self, id: u32) -> bool {
        // FIXME: handle Option properly
        let artifact = self.artifacts.get(&id).unwrap();
        // NEW: { .. } notation to ignore remaining Enum fields
        match &artifact.kind {
            ArtifactKind::Book { .. } | ArtifactKind::AudioBook { .. } => true,
            ArtifactKind::Statue { .. } | ArtifactKind::Painting { .. } => false,
        }
    }

    // QUERYING METHODS
    fn find_artifact_by_title(&self, title: &str) -> &HashSet<u32> {
        self.artifacts_by_title.get(title).unwrap() // FIXME: handle Option properly
    }

    fn find_artifact_by_author(&self, author: &str) -> &HashSet<u32> {
        self.artifacts_by_author.get(author).unwrap() // FIXME: handle Option properly
    }

    fn find_artifact_by_keyword(&self, query: &str) -> Option<HashSet<u32>> {
        // query should be on the following format
        // "keyword1 AND keyword2"
        // "keyword1 OR keyword2"
        // "keyword1 AND keyword2 OR keyword3"

        // NEW: .partition to split iterator in two collections
        let special_tokens = vec!["AND", "OR"];
        let parsed_query = query.split_whitespace();
        let (operators, keywords): (Vec<&str>, Vec<&str>) =
            parsed_query.partition(|token| special_tokens.contains(token));

        // NEW: handle malformed strings
        if operators.is_empty() && keywords.is_empty() {
            return None;
        }
        if operators.len() != keywords.len() - 1 {
            return None; // FIXME: this should be an Error
        }

        // NEW: rewritten with iterators + higher-order functions
        // NEW: .filter_map to handle iterators over Options
        let mut keyword_ids = keywords
            .into_iter()
            .map(|kw| self.artifacts_by_keyword.get(kw)) // Options
            .filter_map(|opt_kw| opt_kw)
            .collect::<Vec<_>>()
            .into_iter();

        let mut acc = HashSet::<u32>::new();
        acc.extend(keyword_ids.next().unwrap()); // FIXME: handle Option

        // NEW: .fold to reduce with explicit initial value
        // NEW: replaced .intersection by .retain
        // NEW: replaced .union by .extend
        let _ = keyword_ids
            .zip(operators)
            .into_iter()
            .fold(&mut acc, |acc, (ids, op)| {
                match op {
                    "AND" => acc.retain(|x| ids.contains(x)),
                    "OR" => acc.extend(ids.iter()),
                    _ => unreachable!(), // FIXME: this should be an Error?
                }
                acc
            });

        Some(acc) // TODO: should this return the ids or the Artifacts?
    }

    fn find_artifact_by_kind(&self, kind: ArtifactKindName) -> &HashSet<u32> {
        self.artifacts_by_kind.get(&kind).unwrap() // FIXME: handle Option properly
    }

    // INTERFACE METHODS
    fn get_artifact_clone_by_id(&self, id: u32) -> Option<Artifact> {
        if let Some(artifact) = self.artifacts.get(&id) {
            return Some(artifact.clone());
        }
        None
    }

    fn add_artifact(&mut self, artifact: Artifact) {
        if self.has_artifact(artifact.id) {
            todo!() // TODO: handle adding Artifact.id that already exists (increase units?)
        } else {
            self.update_reverse_maps_on_add(&artifact);
            self.artifacts.insert(artifact.id, artifact);
        }
    }

    fn update_reverse_maps_on_add(&mut self, artifact: &Artifact) {
        // shared

        self.update_shared_reverse_maps_on_add(artifact);

        // variant-specific
        // TODO: finish this implementation
        match &artifact.kind {
            ArtifactKind::Book { .. } => {}      // isbn, pages
            ArtifactKind::AudioBook { .. } => {} // duration, narrator
            ArtifactKind::Statue { .. } => {}    // dimensions, weight, material
            ArtifactKind::Painting { .. } => {}  // dimensions, style
        }
    }

    fn update_shared_reverse_maps_on_add(&mut self, artifact: &Artifact) {
        // TODO: update Artifact variant inverse map (to be added)
        // TODO: handle Errors properly (no unwraps!)
        // NEW: HashMap.insert returns a bool! true = newly inserted, false = already present
        // NEW: .for_each + scope to discard the bools returned by HashMap.insert
        if let Some(reverse_map) = self.artifacts_by_title.get_mut(&artifact.title) {
            reverse_map.insert(artifact.id);
        } else {
            let mut new = HashSet::new();
            new.insert(artifact.id);
            self.artifacts_by_title.insert(artifact.title.clone(), new);
        }

        if let Some(reverse_map) = self.artifacts_by_author.get_mut(&artifact.author) {
            reverse_map.insert(artifact.id);
        } else {
            let mut new = HashSet::new();
            new.insert(artifact.id);
            self.artifacts_by_author
                .insert(artifact.author.clone(), new);
        }

        for keyword in artifact.keywords.iter() {
            if let Some(reverse_map) = self.artifacts_by_keyword.get_mut(keyword) {
                reverse_map.insert(artifact.id);
            } else {
                let mut new = HashSet::new();
                new.insert(artifact.id);
                self.artifacts_by_keyword.insert(keyword.clone(), new);
            }
        }

        if let Some(reverse_map) = self.artifacts_by_kind.get_mut(&artifact.kind.name()) {
            reverse_map.insert(artifact.id);
        } else {
            let mut new = HashSet::new();
            new.insert(artifact.id);
            self.artifacts_by_kind
                .insert(artifact.kind.name().clone(), new);
        }
    }

    fn increment_artifact_units(&mut self, id: u32, units: u32) {
        // FIXME: handle Option
        // FIXME: handle integer overflow
        self.artifacts.get_mut(&id).unwrap().units += units
    }

    fn decrement_artifact_units(&mut self, id: u32, units: u32) {
        // FIXME: handle Option
        // FIXME: handle Error: units must be non-negative
        self.artifacts.get_mut(&id).unwrap().units -= units
    }

    fn remove_artifact(&mut self, id: u32) {
        if self.has_artifact(id) {
            self.update_reverse_maps_on_remove(id);
            self.artifacts.remove(&id);
            // FIXME: handle Error: artifact not found
        }
    }

    fn update_reverse_maps_on_remove(&mut self, id: u32) {
        // TODO: update Artifact variant inverse map (to be added)
        // TODO: handle Errors properly (no unwraps!)
        // shared
        self.update_shared_reverse_maps_on_remove(id);

        // variant-specific

        // clean up empty HashSets
        self.cleanup_reverse_maps();
    }

    fn update_shared_reverse_maps_on_remove(&mut self, id: u32) {
        let artifact = self.artifacts.get_mut(&id).unwrap(); // FIXME: unwrap is safe?
        self.artifacts_by_title
            .get_mut(&artifact.title)
            .unwrap()
            .remove(&artifact.id);
        self.artifacts_by_author
            .get_mut(&artifact.author)
            .unwrap()
            .remove(&artifact.id);
        for keyword in artifact.keywords.iter() {
            self.artifacts_by_keyword
                .get_mut(keyword)
                .unwrap()
                .remove(&artifact.id);
        }
        self.artifacts_by_kind
            .get_mut(&artifact.kind.name())
            .unwrap()
            .remove(&artifact.id);
    }

    fn cleanup_reverse_maps(&mut self) {
        let reverse_maps = vec![
            &mut self.artifacts_by_title,
            &mut self.artifacts_by_author,
            &mut self.artifacts_by_keyword,
        ];

        // NEW: .retain is like a .filter which modifies HashMap in place!
        for reverse_map in reverse_maps {
            reverse_map.retain(|_, v| !v.is_empty());
        }
    }

    fn lend_artifact(&mut self, id: u32, units: u32) {
        if self.has_available_artifact(id, units) && self.can_lend(id) {
            self.decrement_artifact_units(id, units);
        } // TODO: handle Error: artifact not found / not enough units available / cannot lend
    }

    fn return_artifact(&mut self, id: u32, units: u32) {
        if self.has_artifact(id) && self.can_lend(id) {
            self.increment_artifact_units(id, units);
        } // TODO: handle Error: artifact not found / cannot lend (or return)
    }
}

// testing code
fn create_example_library() -> Library {
    let artifacts = vec![
        Artifact::new(
            1,
            Some(2),
            "The Rust Book",
            "Steve Klabnik",
            vec!["rust", "programming", "learning"],
            ArtifactKind::Book {
                pages: 552,
                isbn: "9781098122539".to_string(),
            },
        ),
        Artifact::new(
            2,
            Some(1),
            "Rust Audio Guide",
            "Carol Nichols",
            vec!["rust", "audio", "learning"],
            ArtifactKind::AudioBook {
                duration_minutes: 320.0,
                narrator: "Carol Nichols".to_string(),
            },
        ),
        Artifact::new(
            3,
            None,
            "Statue of Liberty",
            "Frédéric Auguste Bartholdi",
            vec!["monument", "art", "history"],
            ArtifactKind::Statue {
                material: "Copper".to_string(),
                dimensions_cm: (4000.0, 3800.0),
                height_cm: 9300.0,
                weight_kg: 10000.0,
            },
        ),
        Artifact::new(
            4,
            None,
            "Starry Night",
            "Vincent van Gogh",
            vec!["painting", "art", "history"],
            ArtifactKind::Painting {
                dimensions_cm: (200.0, 100.0),
                style: "Post-Impressionism".to_string(),
                medium: "Oil on canvas".to_string(),
            },
        ),
    ];

    Library::new(artifacts)
}

// CLI CODE

enum SystemMsg {
    Welcome,
    Goodbye,
    OptionsMenu,
    ArtifactKindMenu,
    QueryMenu,
    ArtifactData { artifact: Artifact },
    ArtifactIDs { ids: Vec<u32> },
    IDMustBeUniqueWarning,
    InvalidOptionWarning,
}

impl SystemMsg {
    fn display(&self) {
        match self {
            SystemMsg::Welcome => {
                println!("Welcome to our library!");
            }
            SystemMsg::Goodbye => {
                println!("Come back soon!");
            }
            SystemMsg::OptionsMenu => {
                println!("0. Quit");
                println!("1. List IDs");
                println!("2. Add artifact");
                println!("3. Remove artifact");
                println!("4. Loan artifact");
                println!("5. Return artifact");
                println!("6. Query");
            }
            SystemMsg::ArtifactKindMenu => {
                println!("0. Book");
                println!("1. AudioBook");
                println!("2. Statue");
                println!("3. Painting");
            }
            SystemMsg::QueryMenu => {
                println!("0. By ID");
                println!("1. By title");
                println!("2. By author");
                println!("3. By kind");
                println!("4. By keywords");
            }
            SystemMsg::ArtifactData { artifact } => {
                println!("{:#?}", artifact);
            }
            SystemMsg::ArtifactIDs { ids: data } => {
                println!("Registered IDs: {:?}", data);
            }
            SystemMsg::IDMustBeUniqueWarning => {
                println!("Artifact ID must be unique!");
            }
            SystemMsg::InvalidOptionWarning => {
                println!("Invalid option!");
            }
            _ => {}
        }
    }
}

struct Converter {}
impl Converter {
    fn new() -> Self {
        Self {}
    }

    fn integer(str: String) -> u32 {
        match str.parse::<u32>() {
            Ok(integer) => integer,
            Err(_) => todo!(),
        }
    }

    fn float(str: String) -> f32 {
        match str.parse::<f32>() {
            Ok(float) => float,
            Err(_) => todo!(),
        }
    }
}

enum UserInput {
    Simple { flavor_text: String },
    Option,
    ArtifactID { existing_ids: Vec<u32> },
    ArtifactKind,
    Artifact { existing_ids: Vec<u32> },
    Query,
}

impl UserInput {
    fn user_io(&self) -> Option<String> {
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Some(input.trim().to_string()),
            Err(_) => None,
        }
    }

    fn ask_kind(&self) -> Option<ArtifactKind> {
        match self {
            UserInput::ArtifactKind => {
                SystemMsg::ArtifactKindMenu.display();
                let option = UserInput::Option.ask();
                let option = Converter::integer(option.unwrap());
                let kind = match option {
                    0 => {
                        let pages = UserInput::Simple {
                            flavor_text: "Enter pages: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        let pages = Converter::integer(pages);
                        let isbn = UserInput::Simple {
                            flavor_text: "Enter ISBN: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        Some(ArtifactKind::Book { pages, isbn })
                    }
                    1 => {
                        let duration_minutes = UserInput::Simple {
                            flavor_text: "Enter duration_minutes: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        let duration_minutes = Converter::float(duration_minutes);
                        let narrator = UserInput::Simple {
                            flavor_text: "Enter narrator: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        Some(ArtifactKind::AudioBook {
                            duration_minutes,
                            narrator,
                        })
                    }
                    2 => {
                        let dimensions_cm = {
                            let width = UserInput::Simple {
                                flavor_text: "Enter width_cm: ".to_string(),
                            }
                            .ask()
                            .unwrap();
                            let width = Converter::float(width);
                            let depth = UserInput::Simple {
                                flavor_text: "Enter depth_cm: ".to_string(),
                            }
                            .ask()
                            .unwrap();
                            let depth = Converter::float(depth);
                            (width, depth)
                        };
                        let height_cm = UserInput::Simple {
                            flavor_text: "Enter height_cm: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        let height_cm = Converter::float(height_cm);
                        let weight_kg = UserInput::Simple {
                            flavor_text: "Enter weight_kg: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        let weight_kg = Converter::float(weight_kg);
                        let material = UserInput::Simple {
                            flavor_text: "Enter material: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        Some(ArtifactKind::Statue {
                            dimensions_cm,
                            height_cm,
                            weight_kg,
                            material,
                        })
                    }
                    3 => {
                        let dimensions_cm = {
                            let width = UserInput::Simple {
                                flavor_text: "Enter width_cm: ".to_string(),
                            }
                            .ask()
                            .unwrap();
                            let width = Converter::float(width);
                            let depth = UserInput::Simple {
                                flavor_text: "Enter depth_cm: ".to_string(),
                            }
                            .ask()
                            .unwrap();
                            let depth = Converter::float(depth);
                            (width, depth)
                        };
                        let style = UserInput::Simple {
                            flavor_text: "Enter style: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        let medium = UserInput::Simple {
                            flavor_text: "Enter medium: ".to_string(),
                        }
                        .ask()
                        .unwrap();
                        Some(ArtifactKind::Painting {
                            dimensions_cm,
                            style,
                            medium,
                        })
                    }
                    _ => None,
                };
                kind
            }
            _ => None,
        }
    }

    fn ask_artifact(&self) -> Option<Artifact> {
        match self {
            UserInput::Artifact { existing_ids } => {
                let id = UserInput::ArtifactID {
                    existing_ids: existing_ids.clone(),
                }
                .ask()
                .unwrap(); // TODO: handle Option properly
                let id = Converter::integer(id);

                let units = UserInput::Simple {
                    flavor_text: "Enter units: ".to_string(),
                }
                    .ask()
                    .unwrap(); // TODO: handle Option properly + Check for non-negative
                let units = Converter::integer(units);

                let title = UserInput::Simple {
                    flavor_text: "Enter title: ".to_string(),
                }
                .ask()
                .unwrap(); // TODO: handle Option properly
                let author = UserInput::Simple {
                    flavor_text: "Enter author: ".to_string(),
                }
                .ask()
                .unwrap(); // TODO: handle Option properly

                let keywords = UserInput::Simple {
                    flavor_text: "Enter keywords (whitespace separated): ".to_string(),
                }
                .ask();

                let keywords = keywords?
                    .to_lowercase()
                    .split_whitespace()
                    .map(|s| String::from(s))
                    .collect::<Vec<String>>();

                let kind = UserInput::ArtifactKind.ask_kind().unwrap(); // TODO: handle Option properly

                Some(Artifact {
                    id,
                    units,
                    title,
                    author,
                    keywords,
                    kind,
                })
            }
            _ => None,
        }
    }

    fn ask(&self) -> Option<String> {
        match self {
            UserInput::Simple { flavor_text } => {
                println!("{}", flavor_text);
                self.user_io()
            }
            UserInput::Option => {
                SystemMsg::OptionsMenu.display();
                let option = loop {
                    let option = UserInput::Simple {
                        flavor_text: "Select option: ".to_string(),
                    }
                    .ask()
                    .unwrap(); // FIXME: handle Option properly
                    let option = option.parse::<u32>();
                    if option.is_ok() {
                        break option.unwrap();
                    } else {
                        SystemMsg::InvalidOptionWarning.display();
                        continue;
                    }
                };
                Some(option.to_string())
            }
            UserInput::ArtifactID { existing_ids } => {
                println!("Enter artifact ID: ");
                let id = loop {
                    let input = self.user_io().unwrap().parse::<u32>(); // FIXME: handle Option correctly
                    match input {
                        Ok(id) if existing_ids.contains(&id) => {
                            SystemMsg::IDMustBeUniqueWarning.display();
                            SystemMsg::ArtifactIDs {
                                ids: existing_ids.clone(),
                            };
                        }
                        Ok(id) => {
                            break id;
                        }
                        Err(_) => continue,
                    }
                };
                Some(id.to_string())
            }
            UserInput::Query => {
                todo!()
            }
            _ => todo!(),
        }
    }
}

fn main() {
    let mut lib: Library = create_example_library();

    SystemMsg::Welcome.display();
    loop {
        let option = UserInput::Option.ask();
        let option = Converter::integer(option.unwrap()); // TODO: is unwrap safe?
        match option {
            0 => break,
            1 => SystemMsg::ArtifactIDs {
                ids: lib.list_ids(),
            }
            .display(),
            2 => {
                let artifact = UserInput::Artifact {
                    existing_ids: lib.list_ids(),
                }
                .ask_artifact();
                lib.add_artifact(artifact.unwrap());
            }
            _ => SystemMsg::InvalidOptionWarning.display(),
        }
    }
    SystemMsg::Goodbye.display();
}

// static test code
#[cfg(test)]
mod test {
    use crate::{create_example_library, Artifact, ArtifactKind, ArtifactKindName, Library};

    #[test]
    fn test_add_and_remove() {
        let mut lib: Library = create_example_library();

        assert_eq!(lib.list_ids().len(), 4);
        lib.add_artifact(Artifact::new(
            5,
            Some(3),
            "Introduction to Systems Programming",
            "Jane Doe",
            vec!["systems", "rust", "programming"],
            ArtifactKind::Book {
                pages: 420,
                isbn: "9780201109504".to_string(),
            },
        ));

        assert_eq!(lib.has_artifact(5), true);
        assert_eq!(lib.list_ids().len(), 5);
        println!(">>>> [ADD BOOK] <<<<");
        println!("{:#?}", lib);

        lib.remove_artifact(5);

        assert_eq!(lib.list_ids().len(), 4);
        println!(">>>> [REMOVE BOOK] <<<<");
        println!("{:#?}", lib);
    }

    #[test]
    fn test_lend_and_return() {
        let mut lib: Library = create_example_library();

        // Book -> units = 2 -> units = 1
        lib.lend_artifact(1, 1);
        assert_eq!(lib.has_available_artifact(1, 1), true);

        // Book -> units = 1 -> units = 0
        lib.lend_artifact(1, 1);
        assert_eq!(lib.has_available_artifact(1, 1), false);

        // Book -> units = 0 -> units = 1
        lib.return_artifact(1, 1);
        assert_eq!(lib.has_available_artifact(1, 1), true);

        // Statue -> cannot lend
        lib.lend_artifact(3, 1);
        assert_eq!(lib.has_available_artifact(3, 1), true);

        println!(">>>> [CHECK UNITS] <<<<");
        println!("{:#?}", lib);
    }

    #[test]
    fn test_find_by_title() {
        let mut lib: Library = create_example_library();
        let expected_id: u32 = 2;
        let found_id = lib.find_artifact_by_title("Rust Audio Guide");
        assert_eq!(found_id.len(), 1);

        let found_id = found_id.iter().next().copied().unwrap();
        assert_eq!(found_id, expected_id);

        println!(">>>> [FOUND AUDIOBOOK BY TITLE] <<<<");
        let artifact = lib.get_artifact_clone_by_id(found_id);
        println!("{:#?}", artifact);
    }

    #[test]
    fn test_find_by_author() {
        let mut lib: Library = create_example_library();

        lib.add_artifact(Artifact::new(
            5,
            Some(3),
            "Rust ASMR",
            "Steve Klabnik",
            vec!["systems", "rust", "programming"],
            ArtifactKind::AudioBook {
                duration_minutes: 90.0,
                narrator: "Joe".to_string(),
            },
        ));

        let found_id = lib.find_artifact_by_author("Steve Klabnik");
        assert_eq!(found_id.len(), 2);

        let found_id = found_id.iter().copied().collect::<Vec<u32>>();
        println!(">>>> [FOUND BOOK AND AUDIOBOOK BY AUTHOR] <<<<");
        for id in found_id {
            let artifact = lib.get_artifact_clone_by_id(id);
            println!("{:#?}", artifact);
        }
    }

    #[test]
    fn test_find_by_keyword() {
        let mut lib: Library = create_example_library();
        lib.add_artifact(Artifact::new(
            5,
            Some(2),
            "C Primer",
            "Steve McQueen",
            vec!["c", "advanced", "programming"],
            ArtifactKind::Book {
                pages: 1000,
                isbn: "9780201109504".to_string(),
            },
        ));

        lib.add_artifact(Artifact::new(
            6,
            Some(1),
            "Go Programming for Dummies",
            "Joe Bob",
            vec!["go", "advanced", "programming"],
            ArtifactKind::Book {
                pages: 1000,
                isbn: "9780201109504".to_string(),
            },
        ));

        // case 1: _
        // NEW: HashMap.drain clears the set, returning all elements as iterator
        let found_id = lib.find_artifact_by_keyword("programming");
        let found_id = found_id.unwrap().drain().collect::<Vec<u32>>();
        assert_eq!(found_id.len(), 3);
        println!(">>>> [FOUND WITH KEYWORD 'programming'] <<<<");
        for id in found_id {
            let artifact = lib.get_artifact_clone_by_id(id).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }

        // case 2: OR
        let found_id = lib.find_artifact_by_keyword("go OR c");
        let found_id = found_id.unwrap().drain().collect::<Vec<u32>>();
        assert_eq!(found_id.len(), 2);
        println!(">>>> [FOUND WITH KEYWORD 'go OR c'] <<<<");
        for id in found_id {
            let artifact = lib.get_artifact_clone_by_id(id).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }

        // case 3: AND
        let found_id = lib.find_artifact_by_keyword("rust AND audio");
        let found_id = found_id.unwrap().drain().collect::<Vec<u32>>();
        assert_eq!(found_id.len(), 1);
        println!(">>>> [FOUND WITH KEYWORD 'rust AND audio'] <<<<");
        for id in found_id {
            let artifact = lib.get_artifact_clone_by_id(id).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }

        // case 4: OR -> AND
        let found_id = lib.find_artifact_by_keyword("rust OR go AND programming");
        let found_id = found_id.unwrap().drain().collect::<Vec<u32>>();
        assert_eq!(found_id.len(), 2);
        println!(">>>> [FOUND WITH KEYWORD 'rust OR go AND programming'] <<<<");
        for id in found_id {
            let artifact = lib.get_artifact_clone_by_id(id).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }
    }

    #[test]
    fn test_find_by_kind() {
        let mut lib: Library = create_example_library();

        for name in vec![
            ArtifactKindName::Book,
            ArtifactKindName::AudioBook,
            ArtifactKindName::Statue,
            ArtifactKindName::Painting,
        ] {
            let mut found_id = lib.find_artifact_by_kind(name).clone();
            let found_id = found_id.drain().collect::<Vec<u32>>();
            assert_eq!(found_id.len(), 1);

            println!(">>>> [FOUND WITH KIND {:#?}] <<<<", name);
            let artifact = lib.get_artifact_clone_by_id(found_id[0]).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }
    }
}
