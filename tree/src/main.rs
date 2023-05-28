struct Tree {
    height: usize,
}

impl Tree {
    fn cut_down(&mut self) {
        self.height = 20;
    }

    fn cut(&mut self) {
        if self.height > 50 {
            self.height = self.height - 50;
        } else {
            println!("not tall enough")
        }
    }

    fn add_height(&mut self, extra_height: usize) {
        self.height = self.height + extra_height;
    }

    fn double_height(&mut self) {
        self.height = self.height * 2;
    }
}

fn main() {
    let mut tree = Tree {
        height: 200,
    };
    println!("{}", tree.height);

    tree.cut_down();
    println!("{}", tree.height);

    tree.cut();
    println!("{}", tree.height);

    tree.add_height(100);
    println!("{}", tree.height);

    tree.cut();
    println!("{}", tree.height);

    tree.double_height();
    println!("{}", tree.height);
}