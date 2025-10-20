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
const SEPARATION_DISTANCE: i32 = 24;
const CAR_COLOR_LEFT: (u8, u8, u8) = (0, 255, 255);
const CAR_COLOR_AHEAD: (u8, u8, u8) = (0, 0, 255);
const CAR_COLOR_RIGHT: (u8, u8, u8) = (255, 255, 0);

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
        if !Self::is_overlap(&self.cars, &Car::calculate_initial_position(&location)){
            let car = Car::new(location, destination);
            self.cars.push(car);
        }       
    }

    //check if new car would spawn too close to existing car
    pub fn is_overlap(cars: &Vec<Car>, intial_position: &Point) -> bool{

        let x1 = intial_position.x;
        let y1 = intial_position.y;
        let x2 = x1 + CAR_SIZE;
        let y2 = y1 + CAR_SIZE;

        for car in cars {
            if car.position.x >= x1
                && car.position.x <= x2 + SEPARATION_DISTANCE
                && y1 == car.position.y
            {
                return true;                
            }

            if x1 >= car.position.x
                && x1 <= car.position.x + CAR_SIZE + SEPARATION_DISTANCE
                && y1 == car.position.y
            {
                return true;
            }
            if car.position.y >= y1
                && car.position.y <= y2 + SEPARATION_DISTANCE
                && x1 == car.position.x
            {
                return true;
            }
            if y1 >= car.position.y
                && y1 <= car.position.y + CAR_SIZE + SEPARATION_DISTANCE
                && x1 == car.position.x
            {
                return true;
            }
        }
        false
    }

    //turn car at crossroads
    pub fn update_direction(car: &mut Car) {
        match car.destination {
            Destination::Left => match car.direction {
                Location::West => {
                    if car.position.x <= (SCREEN_WIDTH + MARGIN) / 2 {
                        car.direction = Location::South;
                        car.destination = Destination::Ahead;
                    }
                }
                Location::East => {
                    if car.position.x >= SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN / 2 {
                        car.direction = Location::North;
                        car.destination = Destination::Ahead;
                    }
                }
                Location::North => {
                    if car.position.y <= (SCREEN_HEIGHT + MARGIN) / 2 {
                        car.direction = Location::West;
                        car.destination = Destination::Ahead;
                    }
                }
                Location::South => {
                    if car.position.y >= SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN / 2 {
                        car.direction = Location::East;
                        car.destination = Destination::Ahead;
                    }
                }
            },
            Destination::Right => match car.direction {
                Location::West => {
                    if car.position.x <= SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN / 2 {
                        car.direction = Location::North;
                        car.destination = Destination::Ahead;
                    }
                }
                Location::East => {
                    if car.position.x >= (SCREEN_WIDTH + MARGIN) / 2 {
                        car.direction = Location::South;
                        car.destination = Destination::Ahead;
                    }
                }
                Location::North => {
                    if car.position.y <= SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN / 2 {
                        car.direction = Location::East;
                        car.destination = Destination::Ahead;
                    }
                }
                Location::South => {
                    if car.position.y >= (SCREEN_HEIGHT + MARGIN) / 2 {
                        car.direction = Location::West;
                        car.destination = Destination::Ahead;
                    }
                }
            },
            _ => {}
        }
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

#[derive(Clone, Debug)]
pub struct Car {
    pub position: Point,
    pub size: Dimen,
    pub color: (u8, u8, u8),
    pub destination: Destination,
    pub direction: Location,
}
impl Car{
    pub fn new(location: Location, destination: Destination) -> Self {
        let position = Car::calculate_initial_position(&location);
        let dimen = Dimen::new(CAR_SIZE, CAR_SIZE);
        let (r, g, b) = match destination {
            Destination::Ahead => CAR_COLOR_AHEAD,
            Destination::Right => CAR_COLOR_RIGHT,
            Destination::Left => CAR_COLOR_LEFT,
        };
        let direction = match location {
            Location::East => Location::West,
            Location::West => Location::East,
            Location::North => Location::South,
            Location::South => Location::North,
        };
        Self {
            position,
            size: dimen,
            color: (r, g, b),
            destination,
            direction,
        }
    }
    pub fn calculate_initial_position(location: &Location) -> Point {
        let position = match location {
            Location::West => Point::new(MARGIN, (SCREEN_HEIGHT - CAR_SIZE * 2 - MARGIN) / 2),
            Location::North => Point::new((SCREEN_WIDTH + MARGIN) / 2, MARGIN),
            Location::East => Point::new(
                SCREEN_WIDTH - CAR_SIZE - MARGIN,
                (SCREEN_HEIGHT + MARGIN) / 2,
            ),
            Location::South => Point::new(
                (SCREEN_WIDTH - CAR_SIZE * 2 - MARGIN) / 2,
                SCREEN_HEIGHT - CAR_SIZE - MARGIN,
            ),
        };
        position
    }
    pub fn drive(&mut self, cars: &Vec<Car>) {
        //check separation distance
        //West Side
        if self.direction == Location::East {
            let max_x = self.position.x + CAR_SIZE + SEPARATION_DISTANCE;
            for c in cars {
                if c.direction == Location::East && c.position.x == max_x {
                    return;
                }
            }
        }
        //East Side
        if self.direction == Location::West {
            let max_x = self.position.x - CAR_SIZE - SEPARATION_DISTANCE;
            for c in cars {
                if c.direction == Location::West && c.position.x == max_x {
                    return;
                }
            }
        }
        //North Side
        if self.direction == Location::South {
            let max_y = self.position.y + CAR_SIZE + SEPARATION_DISTANCE;
            for c in cars {
                if c.direction == Location::South && c.position.y == max_y {
                    return;
                }
            }
        }
        //South Side
        if self.direction == Location::North {
            let max_y = self.position.y - CAR_SIZE - SEPARATION_DISTANCE;
            for c in cars {
                if c.direction == Location::North && c.position.y == max_y {
                    return;
                }
            }
        }
        match self.direction {
            Location::East => {
                self.position.x += 1;
            }
            Location::West => {
                self.position.x -= 1;
            }
            Location::North => {
                self.position.y -= 1;
            }
            Location::South => {
                self.position.y += 1;
            }
        }
        Model::update_direction(self);
    }

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


#[derive(Clone, Debug)]
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