extern crate petgraph;

use piet::image;

pub mod blockmap {
    use petgraph::unionfind;

    pub struct BlockMap {
        width: usize,
        height: usize,
        tree: unionfind::UnionFind<usize>,
    }

    pub type index = (usize, usize);

    impl BlockMap {
        pub fn new(width: usize, height: usize) -> BlockMap {
            BlockMap {
                width: width,
                height: height,
                tree: unionfind::UnionFind::<usize>::new(width * height),
            }
        }

        fn integer_of_index(&self, (x, y): index) -> usize {
            y * self.width + x
        }

        fn index_of_integer(&self, n: usize) -> index {
            (n % self.width, n / self.width)
        }

        pub fn find(&self, idx: index) -> index {
            self.index_of_integer(self.tree.find(self.integer_of_index(idx)))
        }

        pub fn union(&mut self, x: index, y: index) -> bool {
            let x = self.integer_of_index(x);
            let y = self.integer_of_index(y);
            self.tree.union(x, y)
        }
    }
}

fn make_block_map(img: &image::Image) -> blockmap::BlockMap {
    let mut map = blockmap::BlockMap::new(100, 100);
    map
}
