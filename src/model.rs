use rand::Rng;
use core::f64;
use std::collections::HashMap;
use crate::constants::*;

pub struct Model {
    pub cars: Vec<Car>,
    pub road_marking: Vec<Line>,
    pub traffic_light_switch: TrafficLightSwitch,    
}

impl Model {
    pub fn new() -> Self {
        let cars = vec![];
        let road_marking = Model::create_road_markings();
        let traffic_lights = TrafficLightSwitch::create_traffic_lights();
        let traffic_light_switch = TrafficLightSwitch {
            traffic_lights,
            request: None,
        };     
        Self {
            cars,
            road_marking,
            traffic_light_switch
        }
    }

    pub fn spawn_car(&mut self, location: Location, destination: Destination) {
        if !Self::is_overlap(&self.cars, &Car::calculate_initial_position(&location)){
            let car = Car::new(location, destination);
            self.cars.push(car);
        }       
    }

    //check if new car would spawn too close to existing car
    pub fn is_overlap(cars: &Vec<Car>, intial_position: &PointF) -> bool{

        let x1 = intial_position.x;
        let y1 = intial_position.y;
        let x2 = x1 + CAR_SIZE_F64;
        let y2 = y1 + CAR_SIZE_F64;

        for car in cars {
            if car.position.x >= x1
                && car.position.x <= x2 + SEPARATION_DISTANCE
                && y1 == car.position.y
            {
                return true;                
            }

            if x1 >= car.position.x
                && x1 <= car.position.x + CAR_SIZE_F64 + SEPARATION_DISTANCE
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
                && y1 <= car.position.y + CAR_SIZE_F64 + SEPARATION_DISTANCE
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
                    if car.position.x <= (SCREEN_WIDTH_F64 + CAR_SIZE_F64 + MARGIN_F64) / 2.0 {
                        // car.direction = Location::South;
                        // car.destination = Destination::Ahead;
                        // car.deg = 90.0;
                    }
                }
                Location::East => {
                    if car.position.x >= (SCREEN_WIDTH_F64  - CAR_SIZE_F64 - MARGIN_F64) / 2.0 {
                        // car.direction = Location::North;
                        // car.destination = Destination::Ahead;
                        // car.deg = 270.0;                     
                    }
                }
                Location::North => {
                    if car.position.y <= (SCREEN_HEIGHT_F64 + CAR_SIZE_F64 + MARGIN_F64) / 2.0 {
                        // car.direction = Location::West;
                        // car.destination = Destination::Ahead;
                        // car.deg = 180.0;
                    }
                }
                Location::South => {
                    if car.position.y >= (SCREEN_HEIGHT_F64 - CAR_SIZE_F64 - MARGIN_F64) / 2.0 {
                        // car.direction = Location::East;
                        // car.destination = Destination::Ahead;
                        // car.deg = 0.0;
                    }
                }
            },
            Destination::Right => match car.direction {
                Location::West => {
                    if car.position.x <= (SCREEN_WIDTH_F64 - CAR_SIZE_F64 - MARGIN_F64) / 2.0 {
                        // car.direction = Location::North;
                        // car.destination = Destination::Ahead;
                        // car.deg = 270.0;
                    }
                }
                Location::East => {
                    if car.position.x >= (SCREEN_WIDTH_F64 + CAR_SIZE_F64 + MARGIN_F64) / 2.0 {
                        // car.direction = Location::South;
                        // car.destination = Destination::Ahead;
                        // car.deg = 90.0;
                    }
                }
                Location::North => {
                    if car.position.y <= (SCREEN_HEIGHT_F64 - CAR_SIZE_F64 - MARGIN_F64) / 2.0 {
                        // car.direction = Location::East;
                        // car.destination = Destination::Ahead;
                        // car.deg = 0.0;
                    }
                }
                Location::South => {
                    if car.position.y >= (SCREEN_HEIGHT_F64 + CAR_SIZE_F64 + MARGIN_F64) / 2.0 {
                        // car.direction = Location::West;
                        // car.destination = Destination::Ahead;
                        // car.deg = 180.0;
                    }
                }
            },
            _ => {}
        }
    }

    pub fn create_road_markings() -> Vec<Line> {
        let mut lines = vec![];
        //Break Point East
        let start = Point::new(BREAK_POINT_EAST as i32, SCREEN_HEIGHT / 2);
        let end = Point::new(BREAK_POINT_EAST as i32, SCREEN_HEIGHT / 2 + CAR_SIZE + MARGIN + STOP_LINE_CURVE_LENGTH_ADJUSTMENT);
        let line = Line {
            start,
            end,
            color: LINE_COLOR_2,
        };
        lines.push(line);
        //Break Point West
        let start = Point::new(BREAK_POINT_WEST as i32, SCREEN_HEIGHT / 2 - CAR_SIZE - MARGIN - STOP_LINE_CURVE_LENGTH_ADJUSTMENT);
        let end = Point::new(BREAK_POINT_WEST as i32, SCREEN_HEIGHT / 2);
        let line = Line {
            start,
            end,
            color: LINE_COLOR_2,
        };
        lines.push(line);
        //Break Point North
        let start = Point::new(SCREEN_WIDTH / 2 + CAR_SIZE + MARGIN + STOP_LINE_CURVE_LENGTH_ADJUSTMENT, BREAK_POINT_NORTH as i32);
        let end = Point::new(SCREEN_WIDTH / 2 , BREAK_POINT_NORTH as i32);
        let line = Line {
            start,
            end,
            color: LINE_COLOR_2,
        };
        lines.push(line);
        //Break Point South
        let start = Point::new(SCREEN_WIDTH / 2 - CAR_SIZE - MARGIN - STOP_LINE_CURVE_LENGTH_ADJUSTMENT, BREAK_POINT_SOUTH as i32);
        let end = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_SOUTH as i32);
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
            let start = Point::new(BREAK_POINT_WEST as i32  - gap, SCREEN_HEIGHT / 2);
            let end = Point::new(BREAK_POINT_WEST as i32  - 50 - gap, SCREEN_HEIGHT / 2);
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
            let start = Point::new(BREAK_POINT_EAST as i32 + gap, SCREEN_HEIGHT / 2);
            let end = Point::new(BREAK_POINT_EAST as i32 + 50 + gap, SCREEN_HEIGHT / 2);
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
            let start = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_NORTH as i32 - gap);
            let end = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_NORTH as i32 - 50 - gap);
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
            let start = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_SOUTH as i32 + gap);
            let end = Point::new(SCREEN_WIDTH / 2, BREAK_POINT_SOUTH as i32 + 50 + gap);
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

    pub fn is_crossing_clear(cars: Vec<Car>) -> bool {

        let p1 = PointF::new(
            SCREEN_WIDTH_F64 / 2.0 - CAR_SIZE_F64 - MARGIN_F64,
            SCREEN_HEIGHT_F64 / 2.0 - CAR_SIZE_F64 - MARGIN_F64,
        );
        let p2 = PointF::new(
            SCREEN_WIDTH_F64 / 2.0 + CAR_SIZE_F64 + MARGIN_F64,
            SCREEN_HEIGHT_F64 / 2.0 + CAR_SIZE_F64 + MARGIN_F64,
        );

        for car in &cars {
            let tl = PointF::new(car.position.x - CAR_SIZE_F64/2.0, car.position.y - CAR_SIZE_F64/2.0);
            let tr = PointF::new(car.position.x + CAR_SIZE_F64/2.0, car.position.y - CAR_SIZE_F64/2.0);
            let bl = PointF::new(car.position.x - CAR_SIZE_F64/2.0, car.position.y + CAR_SIZE_F64/2.0);
            let br = PointF::new(car.position.x + CAR_SIZE_F64/2.0, car.position.y + CAR_SIZE_F64/2.0);
            if tl.x > p1.x && tl.x < p2.x && tl.y > p1.y && tl.y < p2.y {
                return false;
            }
            if bl.x > p1.x && bl.x < p2.x && bl.y > p1.y && bl.y < p2.y {
                return false;
            }
            if tr.x > p1.x && tr.x < p2.x && tr.y > p1.y && tr.y < p2.y {
                return false;
            }
            if br.x > p1.x && br.x < p2.x && br.y > p1.y && br.y < p2.y {
                return false;
            }
        }       
        true
    }

}

#[derive(Clone, Debug)]
pub struct Car {
    pub position: PointF,
    pub size: Dimen,
    pub color: ColorOrUrl,
    pub destination: Destination,
    pub direction: Location,
    pub deg: f64,
    pub rad: f64
}
impl Car{
    pub fn new(location: Location, destination: Destination) -> Self {
        let position = Car::calculate_initial_position(&location);
        let dimen = Dimen::new(CAR_SIZE, CAR_SIZE);
        let color_or_url = match destination {
            Destination::Ahead => ColorOrUrl{color: CAR_COLOR_WHITE, url: String::from(WHITE_CAR_URL)},
            Destination::Right => ColorOrUrl { color: CAR_COLOR_ORANGE, url: String::from(ORANGE_CAR_URL)},
            Destination::Left => ColorOrUrl { color: CAR_COLOR_BLUE, url: String::from(BLUE_CAR_URL)},
        };
        let direction = match location {
            Location::East => Location::West,
            Location::West => Location::East,
            Location::North => Location::South,
            Location::South => Location::North,
        };

        let deg = match location {
            Location::East => 180.0,
            Location::West => 0.0,
            Location::North => 90.0,
            Location::South => 270.0,
        };

        let rad = match location {
            Location::East => 180.0_f64.to_radians(),
            Location::West => 0.0_f64.to_radians(),
            Location::North => 90.0_f64.to_radians(),
            Location::South => 270.0_f64.to_radians(),
        };

        Self {
            position,
            size: dimen,
            color: color_or_url,
            destination,
            direction,
            deg,
            rad
        }
    }
    pub fn calculate_initial_position(location: &Location) -> PointF {
        let position = match location {
            Location::West => PointF::new(0.0, (SCREEN_HEIGHT_F64 - MARGIN_F64 - CAR_SIZE_F64)/2.0),
            Location::North => PointF::new((SCREEN_WIDTH_F64 + MARGIN_F64 + CAR_SIZE_F64)/2.0, 0.0),
            Location::East => PointF::new(SCREEN_WIDTH_F64,(SCREEN_HEIGHT_F64 + MARGIN_F64 + CAR_SIZE_F64)/2.0),
            Location::South => PointF::new((SCREEN_WIDTH_F64 - MARGIN_F64 - CAR_SIZE_F64)/2.0, SCREEN_HEIGHT_F64)
        };

        // let position = match location {
        //     Location::West => PointF::new(MARGIN_F64, (SCREEN_HEIGHT_F64 - CAR_SIZE_F64 * 2.0 - MARGIN_F64) / 2.0),
        //     Location::North => PointF::new((SCREEN_WIDTH_F64 + MARGIN_F64) / 2.0, MARGIN_F64),
        //     Location::East => PointF::new(
        //         SCREEN_WIDTH_F64 - CAR_SIZE_F64 - MARGIN_F64,
        //         (SCREEN_HEIGHT_F64 + MARGIN_F64) / 2.0,
        //     ),
        //     Location::South => PointF::new(
        //         (SCREEN_WIDTH_F64 - CAR_SIZE_F64 * 2.0 - MARGIN_F64) / 2.0,
        //         SCREEN_HEIGHT_F64 - CAR_SIZE_F64 - MARGIN_F64,
        //     ),
        // };
        position
    }
    pub fn drive(&mut self, cars: &Vec<Car>, traffic_lights: &HashMap<Location, TrafficLight>) {
        //check separation distance
        //West Side
        if self.direction == Location::East {
            let max_x = self.position.x + CAR_SIZE_F64 + SEPARATION_DISTANCE;
            for c in cars {
                if c.direction == Location::East && c.position.x == max_x {
                    return;
                }
            }
        }
        //East Side
        if self.direction == Location::West {
            let max_x = self.position.x - CAR_SIZE_F64 - SEPARATION_DISTANCE;
            for c in cars {
                if c.direction == Location::West && c.position.x == max_x {
                    return;
                }
            }
        }
        //North Side
        if self.direction == Location::South {
            let max_y = self.position.y + CAR_SIZE_F64 + SEPARATION_DISTANCE;
            for c in cars {
                if c.direction == Location::South && c.position.y == max_y {
                    return;
                }
            }
        }
        //South Side
        if self.direction == Location::North {
            let max_y = self.position.y - CAR_SIZE_F64 - SEPARATION_DISTANCE;
            for c in cars {
                if c.direction == Location::North && c.position.y == max_y {
                    return;
                }
            }
        }

        //check traffic light
        //West Side
        if self.position.x == SCREEN_WIDTH_F64 / 2.0 - CAR_SIZE_F64*1.5 - MARGIN_F64
            && self.direction == Location::East
            && traffic_lights[&Location::West].status == false
        {
            return;
        }
        //East Side
        if self.position.x == SCREEN_WIDTH_F64 / 2.0 + CAR_SIZE_F64*1.5 + MARGIN_F64
            && self.direction == Location::West
            && traffic_lights[&Location::East].status == false
        {
            return;
        }
        //South side
        if self.position.y == SCREEN_HEIGHT_F64 / 2.0 + CAR_SIZE_F64*1.5 + MARGIN_F64
            && self.direction == Location::North
            && traffic_lights[&Location::South].status == false
        {
            return;
        }
        //North side
        if self.position.y == SCREEN_HEIGHT_F64 / 2.0 - CAR_SIZE_F64 * 1.5 - MARGIN_F64
            && self.direction == Location::South
            && traffic_lights[&Location::North].status == false
        {
            return;
        }

        Model::update_direction(self);

        //check turning position      
        match self.direction {
            Location::East => {
                match self.destination {
                    Destination::Left => {
                        if self.position.x + CAR_SIZE_F64/2.0 >= SCREEN_WIDTH_F64 / 2.0  - CAR_SIZE_F64 - MARGIN_F64 {                         
                            //small radius
                            let r= CAR_SIZE_F64 + MARGIN_F64/2.0;
                            let rad = CAR_SPEED as f64 /r;
                            let deg = rad.to_degrees();
                            self.deg -= deg;
                            self.rad -= rad;

                            if self.deg.abs() >= 90.0{
                                self.direction = Location::North;
                                self.destination = Destination::Ahead;
                                self.deg = 270.0;
                                self.rad = self.deg.to_radians();
                                return;
                            }
                            let dx = r*self.rad.abs().sin();
                            let x = SCREEN_WIDTH_F64 / 2.0 - CAR_SIZE_F64 - MARGIN_F64 - CAR_SIZE_F64/2.0 + dx;                        
                            let dy = r - (self.rad.abs().cos()) * r;                        
                            let y = SCREEN_HEIGHT_F64/2.0 - MARGIN_F64/2.0 - CAR_SIZE_F64/2.0 - dy;
                            self.position.x = x;
                            self.position.y = y;
                            return;

                        }
                    },
                    Destination::Right => {
                        if self.position.x + CAR_SIZE_F64/2.0 >= SCREEN_WIDTH_F64 / 2.0  - CAR_SIZE_F64 - MARGIN_F64 {   
                            //big radius
                            let r = 2.0 * CAR_SIZE_F64 + 1.5 * MARGIN_F64;
                            let rad = CAR_SPEED as f64 /r;
                            let deg = rad.to_degrees();
                            self.deg += deg;
                            self.rad += rad;
                            if self.deg.abs() >= 90.0{
                                self.direction = Location::South;
                                self.destination = Destination::Ahead;
                                self.deg = 90.0;
                                self.rad = self.deg.to_radians();
                                return;
                            }
                            let dx = r*self.rad.sin();
                            let x = SCREEN_WIDTH_F64 / 2.0 - CAR_SIZE_F64 - MARGIN_F64 - CAR_SIZE_F64/2.0 + dx;                        
                            let dy = r - (self.rad.cos()) * r;                        
                            let y = SCREEN_HEIGHT_F64/2.0 - MARGIN_F64/2.0 - CAR_SIZE_F64/2.0 + dy;
                            self.position.x = x;
                            self.position.y = y;
                            return;


                        }
                    },
                    _ => {}
                }
            },
            Location::West => {
                match self.destination{
                    Destination::Left => {
                        if self.position.x - CAR_SIZE_F64/2.0 <= SCREEN_WIDTH_F64 / 2.0  + CAR_SIZE_F64 + MARGIN_F64{
                            //small radius
                            let r= CAR_SIZE_F64 + MARGIN_F64/2.0;
                            let rad = CAR_SPEED as f64 /r;
                            let deg = rad.to_degrees();
                            self.deg -= deg;
                            self.rad -= rad;
                            if self.deg <= 90.0{
                                self.direction = Location::South;
                                self.destination = Destination::Ahead;
                                self.deg = 90.0;
                                self.rad = self.deg.to_radians();
                                return;
                            }
                            let a = 180_f64.to_radians() - self.rad;
                            let dx = r*a.sin();
                            let x = SCREEN_WIDTH_F64 / 2.0 + CAR_SIZE_F64 + MARGIN_F64 + CAR_SIZE_F64/2.0 - dx;                        
                            let dy =  r - r * a.cos();                        
                            let y = SCREEN_HEIGHT_F64/2.0 + MARGIN_F64/2.0 + CAR_SIZE_F64/2.0 + dy;
                            self.position.x = x;
                            self.position.y = y;
                            return;
                        }
                    },
                    Destination::Right => {
                        if self.position.x - CAR_SIZE_F64/2.0 <= SCREEN_WIDTH_F64 / 2.0  + CAR_SIZE_F64 + MARGIN_F64{
                            //big radius
                            let r = 2.0 * CAR_SIZE_F64 + 1.5 * MARGIN_F64;
                            let rad = CAR_SPEED as f64 /r;
                            let deg = rad.to_degrees();
                            self.deg += deg;
                            self.rad += rad;
                            if self.deg.abs() >= 270.0{
                                self.direction = Location::North;
                                self.destination = Destination::Ahead;
                                self.deg = 270.0;
                                self.rad = self.deg.to_radians();
                                return;
                            }
                            let a = (180_f64.to_radians() - self.rad).abs();
                            let dx = r*a.sin();
                            let x = SCREEN_WIDTH_F64 / 2.0 + CAR_SIZE_F64 + MARGIN_F64 + CAR_SIZE_F64/2.0 - dx;
                            let dy =  r - r * a.cos(); 
                            let y = SCREEN_HEIGHT_F64/2.0 + MARGIN_F64/2.0 + CAR_SIZE_F64/2.0 - dy;
                            self.position.x = x;
                            self.position.y = y;
                            return;
                        }

                    },
                    _ => {}
                }
            },
            Location::North => {
                match self.destination{
                    Destination::Left => {
                        if self.position.y - CAR_SIZE_F64/2.0 <= SCREEN_HEIGHT_F64 / 2.0  + CAR_SIZE_F64 + MARGIN_F64{
                            //small radius
                            let r= CAR_SIZE_F64 + MARGIN_F64/2.0;
                            let rad = CAR_SPEED as f64 /r;
                            let deg = rad.to_degrees();
                            self.deg -= deg;
                            self.rad -= rad;
                            if self.deg <= 180.0{
                                self.direction = Location::West;
                                self.destination = Destination::Ahead;
                                self.deg = 180.0;
                                self.rad = self.deg.to_radians();
                                return;
                            }
                            let a = (270_f64.to_radians() - self.rad).abs();
                            let dx = r - r*a.cos();
                            let x = SCREEN_WIDTH_F64/2.0 - MARGIN_F64/2.0 - CAR_SIZE_F64/2.0 - dx;
                            let dy =  r * a.sin(); 
                            let y = SCREEN_HEIGHT_F64 / 2.0 + CAR_SIZE_F64 + MARGIN_F64 + CAR_SIZE_F64/2.0 - dy;                         
                            self.position.x = x;
                            self.position.y = y;
                            return;
                        }
                    },
                    Destination::Right => {
                        if self.position.y - CAR_SIZE_F64/2.0 <= SCREEN_HEIGHT_F64 / 2.0  + CAR_SIZE_F64 + MARGIN_F64{
                            //big radius
                            let r = 2.0 * CAR_SIZE_F64 + 1.5 * MARGIN_F64;
                            let rad = CAR_SPEED as f64 /r;
                            let deg = rad.to_degrees();
                            self.deg += deg;
                            self.rad += rad;
                            if self.deg >= 360.0{
                                self.direction = Location::East;
                                self.destination = Destination::Ahead;
                                self.deg = 0.0;
                                self.rad = self.deg.to_radians();
                                return;
                            }
                            let a = (270_f64.to_radians() - self.rad).abs();
                            let dx = r - r*a.cos();
                            let x = SCREEN_WIDTH_F64/2.0 - MARGIN_F64/2.0 - CAR_SIZE_F64/2.0 + dx;
                            let dy =  r * a.sin(); 
                            let y = SCREEN_HEIGHT_F64 / 2.0 + CAR_SIZE_F64 + MARGIN_F64 + CAR_SIZE_F64/2.0 - dy;                         
                            self.position.x = x;
                            self.position.y = y;
                            return;

                        }  
                    },
                    _ => {}
                }
            },
            Location::South => {
                match self.destination {
                    Destination::Left => {
                        if self.position.y + CAR_SIZE_F64/2.0 >= SCREEN_HEIGHT_F64 / 2.0  - CAR_SIZE_F64 - MARGIN_F64 {
                            //small radius
                            let r= CAR_SIZE_F64 + MARGIN_F64/2.0;
                            let rad = CAR_SPEED as f64 /r;
                            let deg = rad.to_degrees();
                            self.deg -= deg;
                            self.rad -= rad;
                            if self.deg <= 0.0{
                                self.direction = Location::East;
                                self.destination = Destination::Ahead;
                                self.deg = 0.0;
                                self.rad = self.deg.to_radians();
                                return;
                            }
                            let a = (90_f64.to_radians() - self.rad).abs();
                            let dx = r - r * a.cos();
                            let x = SCREEN_WIDTH_F64 / 2.0 + MARGIN_F64/2.0 + CAR_SIZE_F64/2.0 + dx;                        
                            let dy = r*a.sin();                        
                            let y = SCREEN_HEIGHT_F64/2.0 - MARGIN_F64 - CAR_SIZE_F64 - CAR_SIZE_F64/2.0 + dy;
                            self.position.x = x;
                            self.position.y = y;
                            return;
                        }
                    },
                    Destination::Right => {
                        if self.position.y + CAR_SIZE_F64/2.0 >= SCREEN_HEIGHT_F64 / 2.0  - CAR_SIZE_F64 - MARGIN_F64 {
                            //big radius
                            let r = 2.0 * CAR_SIZE_F64 + 1.5 * MARGIN_F64;
                            let rad = CAR_SPEED as f64 /r;
                            let deg = rad.to_degrees();
                            self.deg += deg;
                            self.rad += rad;
                            if self.deg >= 180.0{
                                self.direction = Location::West;
                                self.destination = Destination::Ahead;
                                self.deg = 180.0;
                                self.rad = self.deg.to_radians();
                                return;
                            }
                            let a = (90_f64.to_radians() - self.rad).abs();
                            let dx = r - r * a.cos();
                            let x = SCREEN_WIDTH_F64 / 2.0 + MARGIN_F64/2.0 + CAR_SIZE_F64/2.0 - dx;                        
                            let dy = r*a.sin();                        
                            let y = SCREEN_HEIGHT_F64/2.0 - MARGIN_F64 - CAR_SIZE_F64 - CAR_SIZE_F64/2.0 + dy;
                            self.position.x = x;
                            self.position.y = y;
                            return;

                        }
                    },
                    _ => {}
                }
            } 
        }       

        match self.direction {
            Location::East => {
                self.position.x += CAR_SPEED_F64;
            }
            Location::West => {
                self.position.x -= CAR_SPEED_F64;
            }
            Location::North => {
                self.position.y -= CAR_SPEED_F64;
            }
            Location::South => {
                self.position.y += CAR_SPEED_F64;
            }
        }
       
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
pub struct PointF {
    pub x: f64,
    pub y: f64,
}
impl PointF {
    pub fn new(x: f64, y: f64) -> Self {
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

pub struct TrafficLight {
    pub location: Location,
    //pub position: Point,
    pub size: Dimen,
    pub status: bool,
}
impl TrafficLight {
    pub fn new(/*position: Point,*/ location: Location) -> Self {
        TrafficLight {
            location,
            //position,
            size: Dimen::new(TRAFFIC_LIGHTS_WIDTH, TRAFFIC_LIGHTS_HEIGTH),
            status: false,
        }
    }


}

pub struct TrafficLightSwitch {
    pub request: Option<Location>,
    pub traffic_lights: HashMap<Location, TrafficLight>,
}

impl TrafficLightSwitch{
    pub fn create_traffic_lights() -> HashMap<Location, TrafficLight> {
        let mut lights = HashMap::new();
        lights.insert(
            Location::West,            
            TrafficLight::new(Location::West),
        );
        lights.insert(
            Location::North,
            TrafficLight::new(Location::North),
        );
        lights.insert(
            Location::South,
            TrafficLight::new(Location::South),
        );
        lights.insert(
            Location::East,
            TrafficLight::new(Location::East),
        );
        lights
    }

    pub fn request(&mut self, location: Location) {
        self.request = Some(location);
    }
    pub fn urgent_request(&mut self, location: Location) {
        self.traffic_lights
            .entry(Location::West)
            .and_modify(|v| v.status = false);
        self.traffic_lights
            .entry(Location::East)
            .and_modify(|v| v.status = false);
        self.traffic_lights
            .entry(Location::North)
            .and_modify(|v| v.status = false);
        self.traffic_lights
            .entry(Location::South)
            .and_modify(|v| v.status = false);
        self.request = Some(location);
    }
    pub fn update(&mut self, cars: Vec<Car>) {
        if let Some(location) = &self.request {
            if !self.traffic_lights[&location].status {
                if Model::is_crossing_clear(cars) {
                    self.traffic_lights
                        .entry(Location::West)
                        .and_modify(|v| v.status = false);
                    self.traffic_lights
                        .entry(Location::East)
                        .and_modify(|v| v.status = false);
                    self.traffic_lights
                        .entry(Location::North)
                        .and_modify(|v| v.status = false);
                    self.traffic_lights
                        .entry(Location::South)
                        .and_modify(|v| v.status = false);
                    self.traffic_lights
                        .entry(location.clone())
                        .and_modify(|v| v.status = true);
                    self.request = None;
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColorOrUrl{
    pub color: (u8, u8, u8),
    pub url: String
}