use rand::Rng;

const CAR_SIZE: i32 = 24;
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const MARGIN: i32 = 8;
const LINE_COLOR: (u8, u8, u8) = (150, 150, 150);
const LINE_COLOR_2: (u8, u8, u8) = (66, 66, 66);
const BREAK_POINT_WEST: i32 = SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN;
const BREAK_POINT_EAST: i32 = SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN;
const BREAK_POINT_NORTH: i32 = SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN;
const BREAK_POINT_SOUTH: i32 = SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN;

pub struct Model {
    pub cars: Vec<Car>,
    pub road_marking: Vec<Line>
}

impl Model {
    pub fn new() -> Self {
        let cars = vec![];
        let road_marking = Model::create_road_markings();     
        Self {
            cars,
            road_marking,

        }
    }

    pub fn spawn_car(&mut self, location: Location, destination: Destination) {
        println!("spawned: {:?}, {:?}", location, destination);
    }

    pub fn create_road_markings() -> Vec<Line> {
        let mut lines = vec![];
        //Top1
        let start = Point::new(0, SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN);
        let end = Point::new(
            SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN,
            SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN,
        );
        let line = Line {
            start,
            end,
            color: LINE_COLOR.clone(),
        };
        lines.push(line);
        //Top2
        let start = Point::new(
            SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN,
            SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN,
        );
        let end = Point::new(SCREEN_WIDTH, SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN);
        let line = Line {
            start,
            end,
            color: LINE_COLOR.clone(),
        };
        lines.push(line);
        //Bottom1
        let start = Point::new(0, SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN);
        let end = Point::new(
            SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN,
            SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN,
        );
        let line = Line {
            start,
            end,
            color: LINE_COLOR.clone(),
        };
        lines.push(line);
        //Bottom2
        let start = Point::new(
            SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN,
            SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN,
        );
        let end = Point::new(SCREEN_WIDTH, SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN);
        let line = Line {
            start,
            end,
            color: LINE_COLOR.clone(),
        };
        lines.push(line);
        //Left1
        let start = Point::new(SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN, 0);
        let end = Point::new(
            SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN,
            SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN,
        );
        let line = Line {
            start,
            end,
            color: LINE_COLOR.clone(),
        };
        lines.push(line);
        //Left2
        let start = Point::new(
            SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN,
            SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN,
        );
        let end = Point::new(SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN, SCREEN_HEIGHT);
        let line = Line {
            start,
            end,
            color: LINE_COLOR.clone(),
        };
        lines.push(line);
        //Right1
        let start = Point::new(SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN, 0);
        let end = Point::new(
            SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN,
            SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN,
        );
        let line = Line {
            start,
            end,
            color: LINE_COLOR.clone(),
        };
        lines.push(line);
        //Right2
        let start = Point::new(
            SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN,
            SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN,
        );
        let end = Point::new(SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN, SCREEN_HEIGHT);
        let line = Line {
            start,
            end,
            color: LINE_COLOR.clone(),
        };
        lines.push(line);
        //Break Point East
        let start = Point::new(BREAK_POINT_EAST, SCREEN_HEIGHT / 2);
        let end = Point::new(BREAK_POINT_EAST, SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN);
        let line = Line {
            start,
            end,
            color: LINE_COLOR_2,
        };
        lines.push(line);
        //Break Point West
        let start = Point::new(BREAK_POINT_WEST, SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN);
        let end = Point::new(BREAK_POINT_WEST, SCREEN_HEIGHT / 2);
        let line = Line {
            start,
            end,
            color: LINE_COLOR_2,
        };
        lines.push(line);
        //Break Point North
        let start = Point::new(SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN, BREAK_POINT_NORTH);
        let end = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_NORTH);
        let line = Line {
            start,
            end,
            color: LINE_COLOR_2,
        };
        lines.push(line);
        //Break Point South
        let start = Point::new(SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN, BREAK_POINT_SOUTH);
        let end = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_SOUTH);
        let line = Line {
            start,
            end,
            color: LINE_COLOR_2,
        };
        lines.push(line);
        //Additional markings
        let mut gap = 0;
        for i in 0..10 {
            if i % 2 != 0 {
                gap += 25;
                continue;
            }
            let start = Point::new(BREAK_POINT_WEST - gap, SCREEN_HEIGHT / 2);
            let end = Point::new(BREAK_POINT_WEST - 50 - gap, SCREEN_HEIGHT / 2);
            let line = Line {
                start,
                end,
                color: LINE_COLOR_2,
            };
            lines.push(line);
            gap += 50;
        }
        let mut gap = 0;
        for i in 0..10 {
            if i % 2 != 0 {
                gap += 25;
                continue;
            }
            let start = Point::new(BREAK_POINT_EAST + gap, SCREEN_HEIGHT / 2);
            let end = Point::new(BREAK_POINT_EAST + 50 + gap, SCREEN_HEIGHT / 2);
            let line = Line {
                start,
                end,
                color: LINE_COLOR_2,
            };
            lines.push(line);
            gap += 50;
        }
        let mut gap = 0;
        for i in 0..10 {
            if i % 2 != 0 {
                gap += 25;
                continue;
            }
            let start = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_NORTH - gap);
            let end = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_NORTH - 50 - gap);
            let line = Line {
                start,
                end,
                color: LINE_COLOR_2,
            };
            lines.push(line);
            gap += 50;
        }
        let mut gap = 0;
        for i in 0..10 {
            if i % 2 != 0 {
                gap += 25;
                continue;
            }
            let start = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_SOUTH + gap);
            let end = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_SOUTH + 50 + gap);
            let line = Line {
                start,
                end,
                color: LINE_COLOR_2,
            };
            lines.push(line);
            gap += 50;
        }
        lines
    }

}

#[derive(Clone)]
pub struct Car {
    pub position: Point,
    pub size: Dimen,
    pub color: (u8, u8, u8),
    pub destination: Destination,
    pub direction: Location,
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}


#[derive(Clone)]
pub struct Dimen {
    pub width: i32,
    pub length: i32,
}
impl Dimen {
    pub fn new(width: i32, length: i32) -> Self {
        Self { width, length }
    }
}

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
pub enum Location {
    South,
    North,
    East,
    West,
}
impl Location {
    pub fn get_random() -> Location {
        let r = rand::rng().random_range(0..4);
        match r {
            0 => Location::West,
            1 => Location::East,
            2 => Location::North,
            _ => Location::South,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Destination {
    Ahead,
    Left,
    Right,
}

impl Destination {
    pub fn get_random() -> Destination {
        let r = rand::rng().random_range(0..3);
        match r {
            0 => Destination::Left,
            1 => Destination::Right,
            _ => Destination::Ahead,
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: (u8, u8, u8),
}