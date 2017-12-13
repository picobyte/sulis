use std::io::{Error, ErrorKind};

use resource::Point;
use resource::ResourceBuilder;

use serde_json;

#[derive(Debug)]
pub struct Tile {
    pub id: String,
    pub name: String,
    pub width: i32,
    pub height: i32,

    pub text_display: Vec<char>,
    pub impass: Vec<Point>,
}

impl Tile {
    pub fn new(builder: TileBuilder) -> Result<Tile, Error> {
        let mut display_vec: Vec<char> = Vec::new();
        let mut impass_points: Vec<Point> = Vec::new();

        for (y, xs) in builder.text_display.into_iter().enumerate() {
            if y >= builder.height {
                return Err(Error::new(ErrorKind::InvalidData,
                                      format!("Text display has too many rows, must have '{}'",
                                              builder.height)));
            }
            for (x, disp) in xs.into_iter().enumerate() {
                if x >= builder.width {
                    return Err(
                        Error::new(ErrorKind::InvalidData,
                                   format!("Text display row has too many columns, must have '{}'",
                                           builder.width))
                        );
                }
                display_vec.push(disp);
            }
        }

        for p in builder.impass.into_iter() {
            // allow an empty vector (no impass points)
            if p.len() == 0 { continue; }

            if p.len() != 2 {
                return Err(Error::new(ErrorKind::InvalidData,
                                      "Impass point array length is not equal to 2."));
            }
            let x = *p.get(0).unwrap();
            let y = *p.get(1).unwrap();
            if x >= builder.width || y >= builder.height {
                return Err(
                    Error::new(ErrorKind::InvalidData, 
                               format!("Impass point has coordinate greater than size '{}, {}'",
                                       builder.width, builder.height))
                    );
            }

            impass_points.push(Point::new(x as i32, y as i32));
        }

        Ok(Tile {
            id: builder.id,
            name: builder.name,
            width: builder.width as i32,
            height: builder.height as i32,
            text_display: display_vec,
            impass: impass_points,
        })
    }

    pub fn get_text_display(&self, x: i32, y: i32) -> char {
        *self.text_display.get((x + y * self.width) as usize).unwrap()
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.id == other.id
    }
}

#[derive(Deserialize, Debug)]
pub struct TileBuilder {
    pub id: String,
    pub name: String,
    pub width: usize,
    pub height: usize,

    pub text_display: Vec<Vec<char>>,
    pub impass: Vec<Vec<usize>>,
}

impl ResourceBuilder for TileBuilder {
    fn owned_id(&self) -> String {
        self.id.to_owned()
    }

    fn new(data: &str) -> Result<TileBuilder, Error> {
        let tile: TileBuilder = serde_json::from_str(data)?;

        Ok(tile)
    }
}
