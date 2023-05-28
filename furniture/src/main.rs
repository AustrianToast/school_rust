#[derive(Debug)]
struct Furniture {
    length: usize,
    width: usize,
    pos_x: isize,
    pos_y: isize,
    movable: bool,
}

impl Furniture {
    fn set_pos_x(&mut self, new_pos_x: isize) -> isize {
        if (new_pos_x as usize + self.length) <= ROOM_LENGTH {
            self.pos_x = new_pos_x;
            self.pos_x
        } else {
            println!("invalid movement");
            self.pos_x
        }
    }

    fn set_pos_y(&mut self, new_pos_y: isize) -> isize {
        if (new_pos_y as usize + self.width) <= ROOM_WIDTH {
            self.pos_y = new_pos_y;
            self.pos_y
        } else {
            println!("invalid movement");
            self.pos_y
        }
    }

    fn move_by(&mut self, by_x: isize, by_y: isize) {
        if self.movable {
            /*
            if I wanted pos_x and pos_y to be usize
            
            if by_x.is_negative() {
                self.set_pos_x(self.pos_x - by_x.wrapping_abs() as usize);
            } else {
                self.set_pos_x(self.pos_x + by_x as usize);
            }
            if by_y.is_negative() {
                self.set_pos_y(self.pos_y - by_y.wrapping_abs() as usize);
            } else {
                self.set_pos_y(self.pos_y + by_y as usize);
            }
            */
            if self.pos_x + by_x >= 0 {
                self.set_pos_x(self.pos_x + by_x);
            } else {
                println!("invalid move");
            }
            if self.pos_y + by_y >= 0 {
                self.set_pos_y(self.pos_y + by_y);
            } else {
                println!("invalid move");
            }
        } else {
            println!("not movable");
        }
    }
}

const ROOM_WIDTH: usize = 2000;
const ROOM_LENGTH: usize = 5000;

fn main() {
    let mut desk = Furniture {
        length: 2000,
        width: 800,
        pos_x: 0,
        pos_y: 0,
        movable: true,
    };

    println!("{:?}", desk);
    println!("room width: {}", ROOM_WIDTH);
    println!("room length: {}", ROOM_LENGTH);

    println!("{}", desk.set_pos_x(100));
    println!("{}", desk.set_pos_y(100));
    println!("desk pos x: {}", desk.pos_x);
    println!("desk pos y: {}", desk.pos_y);

    desk.move_by(3000, 3000);
    println!("desk pos x: {}", desk.pos_x);
    println!("desk pos y: {}", desk.pos_y);

    desk.move_by(160, 300);
    println!("desk pos x: {}", desk.pos_x);
    println!("desk pos y: {}", desk.pos_y);

    desk.move_by(-270, -400);
    println!("desk pos x: {}", desk.pos_x);
    println!("desk pos y: {}", desk.pos_y);
}
