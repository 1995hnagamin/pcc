extern crate petgraph;
use piet::image;

pub mod blockmap {
    use petgraph::unionfind::UnionFind;

    pub struct BlockMap {
        width: usize,
        height: usize,
        tree: UnionFind<usize>,
    }

    pub type index = (usize, usize);

    impl BlockMap {
        pub fn new(width: usize, height: usize) -> BlockMap {
            BlockMap {
                width: width,
                height: height,
                tree: UnionFind::<usize>::new(width * height),
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
    for x1 in 0..img.width() {
        for y1 in 0..img.height() {
            let dx = [0, 1];
            let dy = [1, 0];
            for i in 0..2 {
                let (x2, y2) = (x1 + dx[i], y1 + dy[i]);
                if !img.is_inner(x2, y2) {
                    continue;
                }
                if img.at(x1, y1) == img.at(x2, y2) {
                    let _ = map.union((x1, y1), (x2, y2));
                }
            }
        }
    }
    map
}
