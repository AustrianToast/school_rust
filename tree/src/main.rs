struct Tree {
    height: usize,
}

impl Tree {
    fn cut_down(&mut self) -> usize {
        self.height = 20;
        self.height
    }

    fn cut(&mut self) -> usize {
        if self.height > 50 {
            self.height -= 50;
            self.height
        } else {
            println!("not tall enough");
            self.height
        }
    }

    fn add_height(&mut self, extra_height: usize) -> usize {
        self.height += extra_height;
        self.height
    }

    fn double_height(&mut self) -> usize {
        self.height *= 2;
        self.height
    }
}

fn main() {
    let mut tree = Tree {
        height: 200,
    };
    println!("{}", tree.height);
    
    println!("{}", tree.cut_down());

    println!("{}", tree.cut());

    println!("{}", tree.add_height(100));

    println!("{}", tree.cut());

    println!("{}", tree.double_height());
}