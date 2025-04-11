use std::collections::{HashMap, HashSet};
use thiserror::Error;

// Livraria 4.0: Devemos ter funções que funcionem para os objetos correspondentes
// (obter duração de audio book, dimensões de uma estátua/quadro),
// mas que retornem um erro quando tentamos utilizar em objetos que não possuam essas propriedades.

// Deve também adicionar a capacidade de procurar obras pelo titulo e autor.
// Estes métodos devem retornar objetos que podem ser nulos (Opcionais).

// TODO: methods to read variant-specific fields ✅ Done
// TODO: make sure find_* methods return Options ✅ Done
// TODO: make sure fallible methods return Result ✅ Done
// TODO: define meaningful error structures ✅ Done
// TODO: purge the .unwrap calls UNLESS you previously checked unwrap never fails ✅ Done

// FIXME: CLI doesn't include getters, but unit tests cover all of them

use std::io;

// ERRORS
#[derive(Error, Debug)]
enum LibraryError {
    #[error("Field {0} not available for Artifact of type {1}")]
    FieldNotAvailable(String, String),
    #[error("Id {0} is not registered in the library")]
    IdNotAvailable(u32, Vec<u32>),
    #[error("Id {0} is already registered in the library")]
    DuplicatedId(u32, Vec<u32>),
    #[error("Malformed query. Found {0} keywords and {1} operators. Hint: make sure each keyword is separated by an operator!")]
    MalformedQuery(usize, usize),
    #[error(
        "Query contains operator {0}, which is invalid. Hint: AND / OR are the valid operators."
    )]
    UnknownOperator(String),
    #[error("Not enough units available! Requested {0}, available {1}.")]
    NotEnoughUnitsAvailable(u32, u32),
    #[error("Artifact type cannot be loaned")]
    CannotLoan(ArtifactKindName),
}

// TYPES

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ArtifactKindName {
    Book,
    AudioBook,
    Statue,
    Painting,
}

// Not sure what I am doing here, autocomplete is handling it for me 🙏
impl std::fmt::Display for ArtifactKindName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArtifactKindName::Book => write!(f, "Book"),
            ArtifactKindName::AudioBook => write!(f, "AudioBook"),
            ArtifactKindName::Statue => write!(f, "Statue"),
            ArtifactKindName::Painting => write!(f, "Painting"),
        }
    }
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
    artifacts_by_kind: HashMap<ArtifactKindName, HashSet<u32>>,
    // TODO! Implement variant-specific reverse maps to allow querying
}

impl Library {
    fn new(artifacts: Vec<Artifact>) -> Result<Library, LibraryError> {
        let mut instance = Self {
            artifacts: HashMap::<u32, Artifact>::new(),
            artifacts_by_title: HashMap::<String, HashSet<u32>>::new(),
            artifacts_by_author: HashMap::<String, HashSet<u32>>::new(),
            artifacts_by_keyword: HashMap::<String, HashSet<u32>>::new(),
            artifacts_by_kind: HashMap::<ArtifactKindName, HashSet<u32>>::new(),
        };
        for artifact in artifacts {
            // updates reverse maps automatically
            match instance.add_artifact(artifact) {
                Ok(_) => {}
                Err(LibraryError::DuplicatedId(_, _)) => {
                    continue; // skip if duplicated
                }
                _ => panic!(),
            }
        }
        Ok(instance)
    }

    // GETTERS
    // Book
    fn get_pages(&self, artifact: &Artifact) -> Result<u32, LibraryError> {
        match &artifact.kind {
            ArtifactKind::Book { pages, .. } => Ok(*pages),
            other => Err(LibraryError::FieldNotAvailable(
                "pages".to_string(),
                other.name().to_string(),
            )),
        }
    }
    fn get_isbn(&self, artifact: &Artifact) -> Result<String, LibraryError> {
        match &artifact.kind {
            ArtifactKind::Book { isbn, .. } => Ok(isbn.clone()),
            other => Err(LibraryError::FieldNotAvailable(
                "isbn".to_string(),
                other.name().to_string(),
            )),
        }
    }
    // AudioBook
    fn get_duration_minutes(&self, artifact: &Artifact) -> Result<f32, LibraryError> {
        match &artifact.kind {
            ArtifactKind::AudioBook {
                duration_minutes, ..
            } => Ok(*duration_minutes),
            other => Err(LibraryError::FieldNotAvailable(
                "duration_minutes".to_string(),
                other.name().to_string(),
            )),
        }
    }
    fn get_narrator(&self, artifact: &Artifact) -> Result<String, LibraryError> {
        match &artifact.kind {
            ArtifactKind::AudioBook { narrator, .. } => Ok(narrator.clone()),
            other => Err(LibraryError::FieldNotAvailable(
                "narrator".to_string(),
                other.name().to_string(),
            )),
        }
    }
    // Statue
    fn get_dimensions_cm(&self, artifact: &Artifact) -> Result<(f32, f32), LibraryError> {
        // -- Also works for Painting!
        match &artifact.kind {
            ArtifactKind::Statue { dimensions_cm, .. }
            | ArtifactKind::Painting { dimensions_cm, .. } => Ok(*dimensions_cm),
            other => Err(LibraryError::FieldNotAvailable(
                "dimensions_cm".to_string(),
                other.name().to_string(),
            )),
        }
    }

    fn get_height_cm(&self, artifact: &Artifact) -> Result<f32, LibraryError> {
        match &artifact.kind {
            ArtifactKind::Statue { height_cm, .. } => Ok(*height_cm),
            other => Err(LibraryError::FieldNotAvailable(
                "height_cm".to_string(),
                other.name().to_string(),
            )),
        }
    }
    fn get_weight_kg(&self, artifact: &Artifact) -> Result<f32, LibraryError> {
        match &artifact.kind {
            ArtifactKind::Statue { weight_kg, .. } => Ok(*weight_kg),
            other => Err(LibraryError::FieldNotAvailable(
                "weight_kg".to_string(),
                other.name().to_string(),
            )),
        }
    }
    fn get_material(&self, artifact: &Artifact) -> Result<String, LibraryError> {
        match &artifact.kind {
            ArtifactKind::Statue { material, .. } => Ok(material.clone()),
            other => Err(LibraryError::FieldNotAvailable(
                "material".to_string(),
                other.name().to_string(),
            )),
        }
    }
    // Painting
    fn get_style(&self, artifact: &Artifact) -> Result<String, LibraryError> {
        match &artifact.kind {
            ArtifactKind::Painting { style, .. } => Ok(style.clone()),
            other => Err(LibraryError::FieldNotAvailable(
                "style".to_string(),
                other.name().to_string(),
            )),
        }
    }
    fn get_medium(&self, artifact: &Artifact) -> Result<String, LibraryError> {
        match &artifact.kind {
            ArtifactKind::Painting { medium, .. } => Ok(medium.clone()),
            other => Err(LibraryError::FieldNotAvailable(
                "medium".to_string(),
                other.name().to_string(),
            )),
        }
    }

    // AUXILIARY METHODS
    fn list_ids(&self) -> Vec<u32> {
        self.artifacts.keys().copied().collect::<Vec<u32>>()
    }

    fn has_artifact(&self, id: u32) -> bool {
        self.artifacts.contains_key(&id)
    }

    fn has_available_artifact(&self, id: u32, units: u32) -> bool {
        // FIXME: handle Option properly ✅ Done
        // FIXME: available here means enough units, not if you can or cannot loan!

        if self.has_artifact(id) {
            self.artifacts.get(&id).map_or(false, |artifact| {
                artifact.units >= units // check if
            })
        } else {
            false
        }
    }

    fn can_lend(&self, id: u32) -> bool {
        // FIXME: handle Option properly ✅ Done
        // NEW: if id not present, short-circuit to false
        self.artifacts
            .get(&id)
            .map_or(false, |artifact| match &artifact.kind {
                ArtifactKind::Book { .. } | ArtifactKind::AudioBook { .. } => true,
                ArtifactKind::Statue { .. } | ArtifactKind::Painting { .. } => false,
            })
    }

    // QUERYING METHODS
    // NEW: Return Options!
    fn find_artifact_by_title(&self, title: &str) -> Option<&HashSet<u32>> {
        // FIXME: handle Option properly ✅ Done
        self.artifacts_by_title.get(title)
    }

    fn find_artifact_by_author(&self, author: &str) -> Option<&HashSet<u32>> {
        // FIXME: handle Option properly ✅ Done
        self.artifacts_by_author.get(author)
    }

    fn find_artifact_by_keyword(&self, query: &str) -> Result<Option<HashSet<u32>>, LibraryError> {
        // query should be on the following format
        // "keyword1 AND keyword2"
        // "keyword1 OR keyword2"
        // "keyword1 AND keyword2 OR keyword3"

        // NEW: .partition to split iterator in two collections
        let special_tokens = vec!["AND", "OR"];
        let parsed_query = query.split_whitespace();
        let (operators, keywords): (Vec<&str>, Vec<&str>) =
            parsed_query.partition(|token| special_tokens.contains(token));

        //
        if operators.is_empty() && keywords.is_empty() {
            return Ok(None); // Empty query -> Nothing to see
        }
        if operators.len() != keywords.len() - 1 {
            return Err(LibraryError::MalformedQuery(
                keywords.len(),
                operators.len(),
            )); // Malformed query -> Error!
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
        acc.extend(keyword_ids.next().unwrap()); // unwrap is OK because earlier check guarantees iterator has at least 1 element

        // NEW: .fold to reduce with explicit initial value
        // NEW: replaced .intersection by .retain
        // NEW: replaced .union by .extend
        let _ = keyword_ids
            .zip(operators)
            .into_iter()
            .try_fold(&mut acc, |acc, (ids, op)| {
                match op {
                    "AND" => acc.retain(|x| ids.contains(x)),
                    "OR" => acc.extend(ids.iter()),
                    _ => return Err(LibraryError::UnknownOperator(op.to_string())), // FIXME: this should be an Error?
                }
                Ok(acc)
            })?; // if we get an error, try_fold short-circuits and return the error
        Ok(Some(acc))
    }

    fn find_artifact_by_kind(&self, kind: ArtifactKindName) -> Option<&HashSet<u32>> {
        self.artifacts_by_kind.get(&kind) // FIXME: handle Option properly ✅ Done
    }

    // INTERFACE METHODS
    fn get_artifact_clone_by_id(&self, id: u32) -> Option<Artifact> {
        if let Some(artifact) = self.artifacts.get(&id) {
            return Some(artifact.clone());
        }
        None
    }

    fn add_artifact(&mut self, artifact: Artifact) -> Result<(), LibraryError> {
        if self.has_artifact(artifact.id) {
            Err(LibraryError::DuplicatedId(artifact.id, self.list_ids()))
        } else {
            self.update_reverse_maps_on_add(&artifact);
            self.artifacts.insert(artifact.id, artifact);
            Ok(())
        }
    }

    fn update_reverse_maps_on_add(&mut self, artifact: &Artifact) {
        // shared
        self.update_shared_reverse_maps_on_add(artifact);

        // variant-specific
    }

    fn update_shared_reverse_maps_on_add(&mut self, artifact: &Artifact) {
        // TODO: update Artifact variant inverse map (to be added)
        // TODO: handle Errors properly (no unwraps!)
        // NEW: HashMap.insert returns a bool! true = newly inserted, false = already present
        // NEW: .for_each + scope to discard the bools returned by HashMap.insert

        // We don't have ID access errors here, because in those cases we add a new HashSet
        // For now, no further error handling is required and the function is left as is.

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
            self.artifacts_by_kind.insert(artifact.kind.name(), new);
        }
    }

    fn increment_artifact_units(&mut self, id: u32, units: u32) -> Result<(), LibraryError> {
        match self.artifacts.get_mut(&id) {
            Some(artifact) => artifact.units += units,
            None => return Err(LibraryError::IdNotAvailable(id, self.list_ids())),
        }
        Ok(())
    }

    fn decrement_artifact_units(&mut self, id: u32, units: u32) -> Result<(), LibraryError> {
        match self.artifacts.get_mut(&id) {
            Some(artifact) => artifact.units -= units,
            None => return Err(LibraryError::IdNotAvailable(id, self.list_ids())),
        }
        Ok(())
    }

    fn remove_artifact(&mut self, id: u32) -> Result<(), LibraryError> {
        if self.has_artifact(id) {
            self.update_reverse_maps_on_remove(id)?;
            self.artifacts.remove(&id);
            Ok(())
        } else {
            Err(LibraryError::IdNotAvailable(id, self.list_ids()))
        }
    }

    fn update_reverse_maps_on_remove(&mut self, id: u32) -> Result<(), LibraryError> {
        // TODO: update Artifact variant inverse map (to be added)
        // shared
        self.update_shared_reverse_maps_on_remove(id)?;

        // variant-specific

        // clean up empty HashSets
        self.cleanup_reverse_maps();

        Ok(())
    }

    fn update_shared_reverse_maps_on_remove(&mut self, id: u32) -> Result<(), LibraryError> {
        // let artifact = self.artifacts.get_mut(&id).unwrap(); // FIXME: unwrap is safe?
        let artifact = self.artifacts.get_mut(&id);

        if artifact.is_none() {
            return Err(LibraryError::IdNotAvailable(id, self.list_ids()));
        }
        let artifact = artifact.unwrap(); // safe!

        // If these unwraps fail, we are in an irrecoverable state
        // We have a contract that reverse maps are kept updated whenever we add/remove items
        // through the appropriate method.
        // If this contract is broken, we assume an item was incorrectly added/removed by directly
        // modifying the struct, which is forbidden

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

        Ok(())
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

    fn lend_artifact(&mut self, id: u32, units: u32) -> Result<(), LibraryError> {
        if !self.has_artifact(id) {
            return Err(LibraryError::IdNotAvailable(id, self.list_ids()));
        }
        if !self.can_lend(id) {
            return Err(LibraryError::CannotLoan(
                self.get_artifact_clone_by_id(id).unwrap().kind.name(),
            ));
        }
        if !self.has_available_artifact(id, units) {
            return Err(LibraryError::NotEnoughUnitsAvailable(
                units,
                self.get_artifact_clone_by_id(id).unwrap().units,
            ));
        }

        self.decrement_artifact_units(id, units)?;
        Ok(())
    }

    fn return_artifact(&mut self, id: u32, units: u32) -> Result<(), LibraryError> {
        if !self.has_artifact(id) {
            return Err(LibraryError::IdNotAvailable(id, self.list_ids()));
        }
        if !self.can_lend(id) {
            return Err(LibraryError::CannotLoan(
                self.get_artifact_clone_by_id(id).unwrap().kind.name(),
            ));
        }
        self.increment_artifact_units(id, units)?;
        Ok(())
    }
}

// FIXTURES FOR TESTING (CLI + UNIT TESTS)
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

    Library::new(artifacts).expect("Example library is hardcoded, so all IDs must be unique!")
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
    NoMatchesFoundWarning,
    InvalidIDWarning { id: u32, ids: Vec<u32> },
    InvalidArtifactKindWarning { name: ArtifactKindName },
    InvalidUserInputWarning,
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
            SystemMsg::NoMatchesFoundWarning => {
                println!("No matches found!");
            }
            SystemMsg::InvalidIDWarning { id, ids } => {
                println!("Invalid ID provided {}. Registered IDs: {:?}", id, ids);
            }
            SystemMsg::InvalidArtifactKindWarning { name } => {
                println!("Artifact of type {} cannot be loaned", name);
            }
            SystemMsg::InvalidUserInputWarning => {
                println!("Invalid user input. Hint: empty, failed to parse to u32 or f32, ...")
            }
        }
    }
}

// TODO: Converter needs guards against parsing stuff! ✅ DONE
struct Converter {}
impl Converter {
    fn integer(str: String) -> Option<u32> {
        match str.parse::<u32>() {
            Ok(integer) => Some(integer),
            Err(_) => None,
        }
    }

    fn float(str: String) -> Option<f32> {
        match str.parse::<f32>() {
            Ok(float) => Some(float),
            Err(_) => None,
        }
    }
}

struct UserInput {}

impl UserInput {
    fn ask(flavor_text: Option<&str>) -> Option<String> {
        if let Some(flavor_text) = flavor_text {
            println!("{}", flavor_text);
        }
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Some(input.trim().to_string()),
            Err(_) => None,
        }
    }
}


// FIXME: So much duplicated code... But I don't think it is worth it to bother
//        cleaning this up. 🦀🦀🦀
fn main() {
    let mut lib: Library = create_example_library();

    SystemMsg::Welcome.display();
    loop {
        SystemMsg::OptionsMenu.display();
        let input = UserInput::ask(Some("Select an option: "));

        let Some(input) = input else {
            continue;
        };

        let Some(option) = Converter::integer(input) else {
            continue; // handle parsing problems gracefully
        };

        match option {
            0 => break,
            1 => {
                SystemMsg::ArtifactIDs {
                    ids: lib.list_ids(),
                }
                .display();
            }
            2 => {
                let id = loop {
                    let input = UserInput::ask(Some("Enter ID: "));
                    let Some(input) = input else {
                        continue;
                    };
                    let id = Converter::integer(input);
                    let Some(id) = id else {
                        SystemMsg::InvalidUserInputWarning.display();
                        continue;
                    };

                    if lib.list_ids().contains(&id) {
                        SystemMsg::IDMustBeUniqueWarning.display();
                        SystemMsg::ArtifactIDs {
                            ids: lib.list_ids(),
                        }
                        .display();
                        continue;
                    } else {
                        break id;
                    }
                };
                let input = UserInput::ask(Some("Enter units: "));
                let Some(input) = input else {
                    continue;
                };
                let units = Converter::integer(input);
                let Some(units) = units else {
                    SystemMsg::InvalidUserInputWarning.display();
                    continue;
                };
                let title = UserInput::ask(Some("Enter title: ")).unwrap();
                let author = UserInput::ask(Some("Enter author: ")).unwrap();
                let keywords =
                    UserInput::ask(Some("Enter keywords (whitespace separated): ")).unwrap();
                let keywords = keywords
                    .to_lowercase()
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<_>>();
                SystemMsg::ArtifactKindMenu.display();
                let input = UserInput::ask(Some("Select an option: "));
                let Some(input) = input else {
                    continue;
                };
                let kind = Converter::integer(input);
                let Some(kind) = kind else {
                    SystemMsg::InvalidUserInputWarning.display();
                    continue;
                };

                let artifact = match kind {
                    0 => {
                        let input = UserInput::ask(Some("Enter pages: "));
                        let Some(input) = input else {
                            continue;
                        };
                        let pages = Converter::integer(input);
                        let Some(pages) = pages else {
                            SystemMsg::InvalidUserInputWarning.display();
                            continue;
                        };

                        let isbn = UserInput::ask(Some("Enter ISBN: ")).unwrap();

                        Some(Artifact {
                            id,
                            units,
                            title,
                            author,
                            keywords,
                            kind: ArtifactKind::Book { pages, isbn },
                        })
                    }
                    1 => {
                        let input = UserInput::ask(Some("Enter duration (in minutes): "));
                        let Some(input) = input else {
                            continue;
                        };
                        let duration_minutes = Converter::float(input);
                        let Some(duration_minutes) = duration_minutes else {
                            SystemMsg::InvalidUserInputWarning.display();

                            continue;
                        };

                        let narrator = UserInput::ask(Some("Enter narrator: ")).unwrap();
                        Some(Artifact {
                            id,
                            units,
                            title,
                            author,
                            keywords,
                            kind: ArtifactKind::AudioBook {
                                duration_minutes,
                                narrator,
                            },
                        })
                    }
                    2 => {
                        let dimensions_cm = {
                            let input = UserInput::ask(Some("Enter width (in cm): "));
                            let Some(input) = input else {
                                continue;
                            };
                            let width_cm = Converter::float(input);
                            let Some(width_cm) = width_cm else {
                                SystemMsg::InvalidUserInputWarning.display();

                                continue;
                            };

                            let input = UserInput::ask(Some("Enter depth (in cm): "));
                            let Some(input) = input else {
                                continue;
                            };
                            let depth_cm = Converter::float(input);
                            let Some(depth_cm) = depth_cm else {
                                SystemMsg::InvalidUserInputWarning.display();
                                continue;
                            };

                            (width_cm, depth_cm)
                        };

                        let input = UserInput::ask(Some("Enter height (in cm): "));
                        let Some(input) = input else {
                            continue;
                        };
                        let height_cm = Converter::float(input);
                        let Some(height_cm) = height_cm else {
                            SystemMsg::InvalidUserInputWarning.display();

                            continue;
                        };

                        let input = UserInput::ask(Some("Enter weight (in Kg): "));
                        let Some(input) = input else {
                            continue;
                        };
                        let weight_kg = Converter::float(input);
                        let Some(weight_kg) = weight_kg else {
                            SystemMsg::InvalidUserInputWarning.display();

                            continue;
                        };

                        let material = UserInput::ask(Some("Enter material: ")).unwrap();
                        Some(Artifact {
                            id,
                            units,
                            title,
                            author,
                            keywords,
                            kind: ArtifactKind::Statue {
                                dimensions_cm,
                                height_cm,
                                weight_kg,
                                material,
                            },
                        })
                    }
                    3 => {
                        let dimensions_cm = {
                            let input = UserInput::ask(Some("Enter width (in cm): "));
                            let Some(input) = input else {
                                continue;
                            };
                            let width_cm = Converter::float(input);
                            let Some(width_cm) = width_cm else {
                                SystemMsg::InvalidUserInputWarning.display();

                                continue;
                            };

                            let input = UserInput::ask(Some("Enter depth (in cm): "));
                            let Some(input) = input else {
                                continue;
                            };
                            let depth_cm = Converter::float(input);
                            let Some(depth_cm) = depth_cm else {
                                SystemMsg::InvalidUserInputWarning.display();

                                continue;
                            };

                            (width_cm, depth_cm)
                        };
                        let style = UserInput::ask(Some("Enter style: ")).unwrap();
                        let medium = UserInput::ask(Some("Enter medium: ")).unwrap();
                        Some(Artifact {
                            id,
                            units,
                            title,
                            author,
                            keywords,
                            kind: ArtifactKind::Painting {
                                dimensions_cm,
                                style,
                                medium,
                            },
                        })
                    }
                    _ => {
                        SystemMsg::InvalidOptionWarning.display();
                        None
                    }
                };
                if let Some(artifact) = artifact {
                    SystemMsg::ArtifactData {
                        artifact: artifact.clone(),
                    }
                    .display();
                    match lib.add_artifact(artifact) {
                        Ok(_) => continue,
                        Err(error @ LibraryError::DuplicatedId(_, _)) => {
                            println!("{error}")
                        }
                        _ => panic!(),
                    }
                }
            }
            3 => {
                let input = UserInput::ask(Some("Enter ID: "));
                let Some(input) = input else {
                    continue;
                };
                let id = Converter::integer(input);

                let Some(id) = id else {
                    SystemMsg::InvalidUserInputWarning.display();

                    continue;
                };

                match lib.remove_artifact(id) {
                    Ok(_) => continue,
                    Err(LibraryError::IdNotAvailable(id, ids)) => {
                        SystemMsg::InvalidIDWarning { id, ids }.display();
                    }
                    _ => panic!(), // irrecoverable
                }
            }
            4 => {
                let input = UserInput::ask(Some("Enter ID: "));
                let Some(input) = input else {
                    continue;
                };
                let id = Converter::integer(input);
                let Some(id) = id else {
                    SystemMsg::InvalidUserInputWarning.display();

                    continue;
                };

                let input = UserInput::ask(Some("Enter units: "));
                let Some(input) = input else {
                    continue;
                };
                let units = Converter::integer(input);
                let Some(units) = units else {
                    SystemMsg::InvalidUserInputWarning.display();

                    continue;
                };

                match lib.lend_artifact(id, units) {
                    Ok(_) => continue,
                    Err(LibraryError::IdNotAvailable(id, ids)) => {
                        SystemMsg::InvalidIDWarning { id, ids }.display();
                    }
                    Err(LibraryError::CannotLoan(name)) => {
                        SystemMsg::InvalidArtifactKindWarning { name }.display();
                    }
                    Err(error @ LibraryError::NotEnoughUnitsAvailable(_, _)) => {
                        println!("{error}");
                    }
                    _ => panic!(), // irrecoverable
                }
            }
            5 => {
                let input = UserInput::ask(Some("Enter ID: "));
                let Some(input) = input else {
                    continue;
                };
                let id = Converter::integer(input);
                let Some(id) = id else {
                    SystemMsg::InvalidUserInputWarning.display();

                    continue;
                };

                let input = UserInput::ask(Some("Enter units: "));
                let Some(input) = input else {
                    continue;
                };
                let units = Converter::integer(input);
                let Some(units) = units else {
                    SystemMsg::InvalidUserInputWarning.display();

                    continue;
                };

                match lib.return_artifact(id, units) {
                    Ok(_) => continue,
                    Err(LibraryError::IdNotAvailable(id, ids)) => {
                        SystemMsg::InvalidIDWarning { id, ids }.display();
                    }
                    Err(LibraryError::CannotLoan(name)) => {
                        SystemMsg::InvalidArtifactKindWarning { name }.display();
                    }
                    _ => panic!(), // irrecoverable
                }
            }
            6 => {
                SystemMsg::QueryMenu.display();
                let input = UserInput::ask(Some("Select an option: "));
                let Some(input) = input else {
                    continue;
                };
                let query_mode = Converter::integer(input);
                let Some(query_mode) = query_mode else {
                    SystemMsg::InvalidUserInputWarning.display();

                    continue;
                };

                match query_mode {
                    0 => {
                        let input = UserInput::ask(Some("Enter ID: "));
                        let Some(input) = input else {
                            continue;
                        };
                        let id = Converter::integer(input);
                        let Some(id) = id else {
                            SystemMsg::InvalidUserInputWarning.display();

                            continue;
                        };

                        let artifact = lib.get_artifact_clone_by_id(id).unwrap();
                        SystemMsg::ArtifactData { artifact }.display();
                    }
                    1 => {
                        let title = UserInput::ask(Some("Enter title: ")).unwrap();
                        let ids = lib.find_artifact_by_title(title.as_str());
                        if let Some(ids) = ids {
                            ids.iter().copied().for_each(|id| {
                                SystemMsg::ArtifactData {
                                    artifact: lib.get_artifact_clone_by_id(id).unwrap(),
                                }
                                .display();
                            })
                        } else {
                            SystemMsg::NoMatchesFoundWarning.display();
                            return ();
                        }
                    }
                    2 => {
                        let author = UserInput::ask(Some("Enter author: ")).unwrap();
                        let ids = lib.find_artifact_by_author(author.as_str());
                        if let Some(ids) = ids {
                            ids.iter().copied().for_each(|id| {
                                SystemMsg::ArtifactData {
                                    artifact: lib.get_artifact_clone_by_id(id).unwrap(),
                                }
                                .display();
                            })
                        } else {
                            SystemMsg::NoMatchesFoundWarning.display();
                            return ();
                        }
                    }
                    3 => {
                        SystemMsg::ArtifactKindMenu.display();
                        let input = UserInput::ask(Some("Select an option: "));
                        let Some(input) = input else {
                            continue;
                        };
                        let artifact_kind_option = Converter::integer(input);
                        let Some(artifact_kind_option) = artifact_kind_option else {
                            SystemMsg::InvalidUserInputWarning.display();

                            continue;
                        };

                        let name = match artifact_kind_option {
                            0 => Some(ArtifactKindName::Book),
                            1 => Some(ArtifactKindName::AudioBook),
                            2 => Some(ArtifactKindName::Statue),
                            3 => Some(ArtifactKindName::Painting),
                            _ => {
                                SystemMsg::InvalidOptionWarning.display();
                                None
                            }
                        };
                        if let Some(name) = name {
                            let ids = lib.find_artifact_by_kind(name);

                            if let Some(ids) = ids {
                                ids.iter().copied().for_each(|id| {
                                    SystemMsg::ArtifactData {
                                        artifact: lib.get_artifact_clone_by_id(id).unwrap(),
                                    }
                                    .display();
                                })
                            } else {
                                SystemMsg::NoMatchesFoundWarning.display();
                                return ();
                            }
                        }
                    }
                    4 => {
                        let query = UserInput::ask(Some(
                            "Enter query (whitespace separated | supports AND/OR operators): ",
                        ))
                        .unwrap();
                        let ids = lib.find_artifact_by_keyword(query.as_str()).unwrap(); // FIXME: handle Option properly

                        if let Some(ids) = ids {
                            ids.iter().copied().for_each(|id| {
                                SystemMsg::ArtifactData {
                                    artifact: lib.get_artifact_clone_by_id(id).unwrap(),
                                }
                                .display();
                            })
                        } else {
                            SystemMsg::NoMatchesFoundWarning.display();
                            return ();
                        }
                    }
                    _ => {
                        SystemMsg::InvalidOptionWarning.display();
                    }
                }
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
        ))
        .unwrap();

        assert!(lib.has_artifact(5));
        assert_eq!(lib.list_ids().len(), 5);
        println!(">>>> [ADD BOOK] <<<<");
        println!("{:#?}", lib);

        match lib.remove_artifact(5) {
            Ok(_) => {}
            Err(_) => panic!(),
        }

        assert_eq!(lib.list_ids().len(), 4);
        println!(">>>> [REMOVE BOOK] <<<<");
        println!("{:#?}", lib);
    }

    #[test]
    fn test_lend_and_return() {
        let mut lib: Library = create_example_library();

        // Book -> units = 2 -> units = 1
        match lib.lend_artifact(1, 1) {
            Ok(_) => {}
            Err(_) => panic!(),
        }
        assert!(lib.has_available_artifact(1, 1));

        // Book -> units = 1 -> units = 0
        match lib.lend_artifact(1, 1) {
            Ok(_) => {}
            Err(_) => panic!(),
        }
        assert!(!lib.has_available_artifact(1, 1)); // false => unavailable

        // Book -> units = 0 -> units = 1
        match lib.return_artifact(1, 1) {
            Ok(_) => {}
            Err(_) => panic!(),
        }
        assert!(lib.has_available_artifact(1, 1));

        // Statue -> cannot lend
        assert!(!lib.can_lend(3));
        assert!(lib.has_available_artifact(3, 1)); // cannot lend BUT there are units

        println!(">>>> [CHECK UNITS] <<<<");
        println!("{:#?}", lib);
    }

    #[test]
    fn test_find_by_title() {
        let lib: Library = create_example_library();
        let expected_id: u32 = 2;
        let found_id = lib.find_artifact_by_title("Rust Audio Guide");
        assert!(found_id.is_some());

        let found_id = found_id.unwrap();
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
        ))
        .unwrap();

        let found_id = lib.find_artifact_by_author("Steve Klabnik");
        assert!(found_id.is_some());

        let found_id = found_id.unwrap();
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
        ))
        .unwrap();

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
        ))
        .unwrap();

        // case 1: _
        // NEW: HashMap.drain clears the set, returning all elements as iterator
        let found_id = lib.find_artifact_by_keyword("programming");
        assert!(found_id.is_ok());
        let found_id = found_id.unwrap();
        assert!(found_id.is_some());
        let found_id = found_id.unwrap().drain().collect::<Vec<u32>>();
        assert_eq!(found_id.len(), 3);
        println!(">>>> [FOUND WITH KEYWORD 'programming'] <<<<");
        for id in found_id {
            let artifact = lib.get_artifact_clone_by_id(id).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }

        // case 2: OR
        let found_id = lib.find_artifact_by_keyword("go OR c");
        assert!(found_id.is_ok());
        let found_id = found_id.unwrap();
        assert!(found_id.is_some());
        let found_id = found_id.unwrap().drain().collect::<Vec<u32>>();
        assert_eq!(found_id.len(), 2);
        println!(">>>> [FOUND WITH KEYWORD 'go OR c'] <<<<");
        for id in found_id {
            let artifact = lib.get_artifact_clone_by_id(id).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }

        // case 3: AND
        let found_id = lib.find_artifact_by_keyword("rust AND audio");
        assert!(found_id.is_ok());
        let found_id = found_id.unwrap();
        assert!(found_id.is_some());
        let found_id = found_id.unwrap().drain().collect::<Vec<u32>>();
        assert_eq!(found_id.len(), 1);
        println!(">>>> [FOUND WITH KEYWORD 'rust AND audio'] <<<<");
        for id in found_id {
            let artifact = lib.get_artifact_clone_by_id(id).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }

        // case 4: OR -> AND
        let found_id = lib.find_artifact_by_keyword("rust OR go AND programming");
        assert!(found_id.is_ok());
        let found_id = found_id.unwrap();
        assert!(found_id.is_some());
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
        let lib: Library = create_example_library();

        // Clippy: vec! -> useless, use array instead
        for name in [
            ArtifactKindName::Book,
            ArtifactKindName::AudioBook,
            ArtifactKindName::Statue,
            ArtifactKindName::Painting,
        ] {
            // Note: cloning is needed to end the immut. borrow to lib inside the loop
            //       Otherwise, call to .drain will try to borrow mutably => 💀 death
            let found_id = lib.find_artifact_by_kind(name);
            assert!(found_id.is_some());
            let found_id = found_id.unwrap(); // clone the HashSet, not the Option!
            let found_id = found_id.clone().drain().collect::<Vec<u32>>();
            assert_eq!(found_id.len(), 1);

            println!(">>>> [FOUND WITH KIND {:#?}] <<<<", name);
            let artifact = lib.get_artifact_clone_by_id(found_id[0]).unwrap();
            println!("Title: {:#?}, Kind: {:#?}", &artifact.title, &artifact.kind);
        }
    }

    #[test]
    fn test_getters() {
        let lib: Library = create_example_library();

        // Book
        let book = lib.get_artifact_clone_by_id(1).unwrap();
        assert!(lib.get_pages(&book).is_ok());
        assert!(lib.get_isbn(&book).is_ok());
        assert!(lib.get_duration_minutes(&book).is_err());

        // AudioBook
        let audio_book = lib.get_artifact_clone_by_id(2).unwrap();
        assert!(lib.get_duration_minutes(&audio_book).is_ok());
        assert!(lib.get_narrator(&audio_book).is_ok());
        assert!(lib.get_isbn(&audio_book).is_err());

        // Statue
        let statue = lib.get_artifact_clone_by_id(3).unwrap();
        assert!(lib.get_dimensions_cm(&statue).is_ok());
        assert!(lib.get_height_cm(&statue).is_ok());
        assert!(lib.get_weight_kg(&statue).is_ok());
        assert!(lib.get_material(&statue).is_ok());
        assert!(lib.get_style(&statue).is_err());

        // Painting
        let painting = lib.get_artifact_clone_by_id(4).unwrap();
        assert!(lib.get_dimensions_cm(&painting).is_ok()); // shares this with Statue
        assert!(lib.get_style(&painting).is_ok());
        assert!(lib.get_medium(&painting).is_ok());
        assert!(lib.get_pages(&painting).is_err());
    }
}
