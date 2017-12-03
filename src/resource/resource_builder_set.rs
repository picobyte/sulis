use resource::size::SizeBuilder;
use resource::Game;
use resource::TileBuilder;
use resource::ActorBuilder;
use resource::ResourceBuilder;
use resource::area::AreaBuilder;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Error};
use std::ffi::OsStr;

#[derive(Debug)]
pub struct ResourceBuilderSet {
    pub game: Game,
    pub size_builders: HashMap<String, SizeBuilder>,
    pub area_builders: HashMap<String, AreaBuilder>,
    pub tile_builders: HashMap<String, TileBuilder>,
    pub actor_builders: HashMap<String, ActorBuilder>,
}

impl ResourceBuilderSet {
    pub fn new(root: &str) -> Result<ResourceBuilderSet, Error> {
        let game_filename = root.to_owned() + "/game.json";
        let game = match ResourceBuilderSet::create_game(&game_filename) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Unable to load game startup state from {}", game_filename);
                return Err(e);
            }
        };

        Ok(ResourceBuilderSet {
            game,
            size_builders: read_resources(&format!("{}/sizes/", root)),
            tile_builders: read_resources(&format!("{}/tiles/", root)),
            actor_builders: read_resources(&format!("{}/actors/", root)),
            area_builders: read_resources(&format!("{}/areas/", root)),
        })
    }

    pub fn create_game(filename: &str) -> Result<Game, Error> {
        let mut f = File::open(filename)?;
        let mut file_data = String::new();
        f.read_to_string(&mut file_data)?;
        let game = Game::new(&file_data)?;

        Ok(game)
    }
}

fn read_resources<T: ResourceBuilder>(dir: &str) -> HashMap<String, T> {
    let mut resources: HashMap<String, T> = HashMap::new();
    let dir_entries = fs::read_dir(dir);
    let dir_entries = match dir_entries {
        Ok(entries) => entries,
        Err(_) => {
            eprintln!("Unable to read directory: {}", dir);
            return resources;
        }
    };

    for entry in dir_entries {
        let entry = match entry {
            Ok(e) => e,
            Err(error) => {
                eprintln!("Error reading file: {}", error);
                continue;
            }
        };

        let path = entry.path();
        let path2 = path.clone();
        let extension: &str = match path2.extension() {
            Some(ext) => match OsStr::to_str(ext) {
                Some(str) => str,
                None => ""
            },
            None => ""
        };
        if path.is_file() && extension == "json" {
            let path_str = path.to_string_lossy().into_owned();
            let f = File::open(path);
            let mut f = match f {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("Error reading file: {}", error);
                    continue;
                }
            };

            let mut file_data = String::new();
            if f.read_to_string(&mut file_data).is_err() {
                eprintln!("Error reading file data from file");
                continue;
            }

            let resource = T::new(&file_data); 
            let resource = match resource {
                Ok(a) => a,
                Err(error) => {
                    eprintln!("Error parsing file data: {:?}", path_str);
                    eprintln!("  {}", error);
                    continue;
                }
            };

            let id = resource.owned_id();
            if resources.contains_key(&id) {
                eprintln!("Error: duplicate resource key: {} in {}", id, dir);
                continue;
            }


            resources.insert(id, resource);
        }
    }

    resources
}
