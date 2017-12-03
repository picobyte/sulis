use std::io::{Error, ErrorKind};

use resource::ResourceBuilder;
use resource::Point;

use serde_json;

pub struct Size {
    pub size: usize,
    relative_points: Vec<Point>,
}

impl Size {
    pub fn new(builder: SizeBuilder) -> Result<Size, Error> {
        let mut points: Vec<Point> = Vec::new();
        
        for p in builder.relative_points.into_iter() {
            if p.len() != 2 {
                return Err(Error::new(ErrorKind::InvalidData,
                                      "Point array length is not equal to 2."));
            }
            let x = *p.get(0).unwrap();
            let y = *p.get(1).unwrap();
            if x >= builder.size || y >= builder.size {
                return Err(Error::new(ErrorKind::InvalidData,
                                      format!("Point has coordinate greater than size '{}'",
                                              builder.size)));
            }

            points.push(Point::new(x, y));
        }

        Ok(Size {
            size: builder.size,
            relative_points: points
        })
    }

    pub fn relative_points(&self) -> SizeIterator {
        SizeIterator { size: &self, index: 0, x_offset: 0, y_offset: 0 }
    }

    pub fn points(&self, x: usize, y: usize) -> SizeIterator {
        SizeIterator { size: &self, index: 0, x_offset: x, y_offset: y }
    }
}

pub struct SizeIterator<'a> {
    size: &'a Size,
    index: usize,
    x_offset: usize,
    y_offset: usize,
}

impl<'a> Iterator for SizeIterator<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        let next = self.size.relative_points.get(self.index);

        self.index += 1;

        match next {
            None => None,
            Some(p) => Some(p.add(self.x_offset, self.y_offset))
        }
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Size) -> bool {
        self.size == other.size
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SizeBuilder {
    pub size: usize,
    pub relative_points: Vec<Vec<usize>>,
}

impl ResourceBuilder for SizeBuilder {
    fn owned_id(&self) -> String {
        self.size.to_string()
    }

    fn new(data: &str) -> Result<SizeBuilder, Error> {
        let size: SizeBuilder = serde_json::from_str(data)?;

        Ok(size)
    }
}

