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
    fn get_artifact_clone_by_id(&self, id: u32) -> Option<Artifact> {
        if let Some(artifact) = self.artifacts.get(&id) {
            return Some(artifact.clone());
        }
        None
    }

    fn list_ids(&self) -> Vec<&u32> {
        self.artifacts.keys().collect::<Vec<&u32>>()
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

// cli code
fn main() {}

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
