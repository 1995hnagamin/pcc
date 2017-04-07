extern crate petgraph;

use piet::image;

pub mod blockmap {
    use petgraph::unionfind;

    pub struct BlockMap {
        width: usize,
        height: usize,
        tree: unionfind::UnionFind<usize>,
    }

    impl BlockMap {
        pub fn new(width: usize, height: usize) -> BlockMap {
            BlockMap {
                width: width,
                height: height,
                tree: unionfind::UnionFind::<usize>::new(width * height),
            }
        }
    }
}

fn make_block_map(img: &image::Image) -> blockmap::BlockMap {
    let mut map = blockmap::BlockMap::new(100, 100);
    map
}
