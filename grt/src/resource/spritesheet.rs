use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

use resource::ResourceBuilder;
use util::{Point, Size};

use serde_json;
use serde_yaml;

use extern_image::{self, ImageBuffer, Rgba};

#[derive(Debug)]
pub struct Spritesheet {
    pub id: String,
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub sprites: HashMap<String, Rc<Sprite>>,
}

#[derive(Debug)]
pub struct Sprite {
    pub id: String,
    pub position: Point,
    pub size: Size,
    pub tex_coords: [f32; 8],
}

impl Sprite {
    fn new(id: &str, image_size: &Size, position: Point, size: Size) -> Sprite {
        let image_width = image_size.width as f32;
        let image_height = image_size.height as f32;
        let x_min = (position.x as f32) / image_width;
        let y_min = (image_height - (position.y + size.height) as f32) / image_height;
        let x_max = (position.x + size.width) as f32 / image_width;
        let y_max = (image_height - position.y as f32) / image_height;

        Sprite {
            id: id.to_string(),
            position,
            size,
            tex_coords: [ x_min, y_max,
                          x_min, y_min,
                          x_max, y_max,
                          x_max, y_min ],
        }
    }
}

impl Spritesheet {
    pub fn new(dir: &str, builder: SpritesheetBuilder) -> Result<Rc<Spritesheet>, Error> {
        let filename = format!("{}{}", dir, builder.src);
        let image = match extern_image::open(&filename) {
            Ok(image) => image,
            Err(e) => {
                warn!("Error reading '{}', {}", &filename, e);
                return Err(Error::new(ErrorKind::InvalidData,
                                      format!("Cannot open spritesheet at '{}'", filename)));
            }
        };

        let image = image.to_rgba();
        let (image_width, image_height) = image.dimensions();
        let image_size = Size::new(image_width as i32, image_height as i32);

        let mut sprites: HashMap<String, Rc<Sprite>> = HashMap::new();
        for (_id, group) in builder.groups {
            let base_size = group.get_size();
            let base_pos = group.get_position();
            for (id, area_pos) in group.areas {
                let (pos, size) = match area_pos.len() {
                    2 => (base_pos.add(area_pos[0], area_pos[1]), base_size),
                    4 => (base_pos.add(area_pos[0], area_pos[1]), base_size.add(area_pos[2], area_pos[3])),
                    _ => {
                        warn!("Error in definition for sprite '{}' in sheet '{}'", id, builder.id);
                        warn!("Coordinates must either by [x, y] or [x, y, w, h]");
                        continue;
                    }
                };

                trace!("Creating sprite with id '{}' in '{}'", id, builder.id);
                let sprite = Sprite::new(&builder.id, &image_size, pos, size);

                if sprites.contains_key(&id) {
                    warn!("Duplicate sprite ID in sheet '{}': '{}'", builder.id, id);
                    continue;
                }

                let upper_bound_pos = sprite.position.add(sprite.size.width, sprite.size.height);

                if !sprite.position.in_bounds(image_width as i32 + 1, image_height as i32 + 1) ||
                    !upper_bound_pos.in_bounds(image_width as i32 + 1, image_height as i32 + 1) {
                        warn!("Sprite '{}' in sheet '{}' coordinates fall outside image bounds",
                              id, builder.id);
                        continue;
                    }

                sprites.insert(id, Rc::new(sprite));
            }
        }

        Ok(Rc::new(Spritesheet {
            id: builder.id,
            image,
            sprites,
        }))
    }
}

#[derive(Deserialize, Debug)]
pub struct SpritesheetBuilder {
    pub id: String,
    pub src: String,
    pub size: Size,
    groups: HashMap<String, SpritesheetGroup>,
}

#[derive(Deserialize, Debug)]
struct SpritesheetGroup {
    pub size: Option<Size>,
    pub position: Option<Point>,
    pub areas: HashMap<String, Vec<i32>>,
}

impl SpritesheetGroup {
    fn get_size(&self) -> Size {
        self.size.unwrap_or(Size::as_zero())
    }

    fn get_position(&self) -> Point {
        self.position.unwrap_or(Point::as_zero())
    }
}

impl ResourceBuilder for SpritesheetBuilder {
    fn owned_id(&self) -> String {
        self.id.to_owned()
    }

    fn from_json(data: &str) -> Result<SpritesheetBuilder, Error> {
        let resource: SpritesheetBuilder = serde_json::from_str(data)?;

        Ok(resource)
    }

    fn from_yaml(data: &str) -> Result<SpritesheetBuilder, Error> {
        let resource: Result<SpritesheetBuilder, serde_yaml::Error> = serde_yaml::from_str(data);

        match resource {
            Ok(resource) => Ok(resource),
            Err(error) => Err(Error::new(ErrorKind::InvalidData, format!("{}", error)))
        }
    }
}
