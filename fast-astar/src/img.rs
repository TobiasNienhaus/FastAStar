use crate::{Graph, Node};
use graphlib::VertexId;
use image::{io::Reader as ImgReader, GrayImage, Pixel};
use std::io::Result as IoResult;
use std::path::PathBuf;

type Grid<T> = Vec<Vec<T>>;
type BGrid = Grid<bool>;
type IdGrid = Grid<Option<VertexId>>;

pub mod traits {
    use super::BGrid;
    use super::Grid;
    use super::IdGrid;
    pub trait GridAccess<'a> {
        type Inner;
        fn get_2d(&'a self, pos: (usize, usize)) -> &'a Self::Inner;
        fn get_mut_2d(&'a mut self, pos: (usize, usize)) -> &'a mut Self::Inner;
        fn width(&self) -> usize;
        fn height(&self) -> usize;
    }

    pub(super) trait GridInit {
        fn init(w: usize, h: usize) -> Self;
    }

    impl<'a, T> GridAccess<'a> for Grid<T>
    where
        T: Clone,
    {
        type Inner = T;

        fn get_2d(&'a self, pos: (usize, usize)) -> &'a T {
            self.get(pos.1).unwrap().get(pos.0).unwrap()
        }

        fn get_mut_2d(&'a mut self, pos: (usize, usize)) -> &'a mut Self::Inner {
            self.get_mut(pos.1).unwrap().get_mut(pos.0).unwrap()
        }

        fn width(&self) -> usize {
            self.get(0).unwrap().len()
        }
        fn height(&self) -> usize {
            self.len()
        }
    }

    impl GridInit for BGrid {
        fn init(w: usize, h: usize) -> BGrid {
            vec![vec![false; w]; h]
        }
    }

    impl GridInit for IdGrid {
        fn init(w: usize, h: usize) -> IdGrid {
            vec![vec![None; w]; h]
        }
    }
}

use traits::*;

fn load_luma8(name: PathBuf) -> IoResult<GrayImage> {
    Ok(ImgReader::open(name)?
        .with_guessed_format()?
        .decode()
        .unwrap()
        .to_luma8())
}

fn grid_from_img(img: &GrayImage, cutoff: u8) -> BGrid {
    let mut grid = BGrid::init(img.width() as usize, img.height() as usize);
    for (y, row) in img.rows().enumerate() {
        for (x, pix) in row.enumerate() {
            if pix
                .channels()
                .iter()
                .fold(false, |init, rolling| init | (*rolling > cutoff))
            {
                *grid.get_mut_2d((x, y)) = true
            }
        }
    }
    grid
}

pub fn test_img(name: PathBuf) -> IoResult<()> {
    let img = load_luma8(name)?;
    for (_, row) in img.rows().enumerate() {
        for (_, pix) in row.enumerate() {
            for channel in pix.channels() {
                print!("{:^3} ", channel);
            }
        }
        println!();
    }
    Ok(())
}

pub fn test_grid(name: PathBuf, cutoff: u8) -> IoResult<()> {
    let img = load_luma8(name)?;
    let grid = grid_from_img(&img, cutoff);
    println!("Img dim:  {}x{}", img.width(), img.height());
    println!("Grid dim: {}x{}", grid.len(), grid.get(0).unwrap().len());
    for row in grid {
        for val in row {
            print!("{}", if val { "██" } else { "  " });
        }
        println!();
    }
    Ok(())
}

pub fn load_graph_from_img(path: PathBuf) -> IoResult<(IdGrid, Graph)> {
    load_graph_from_img_with_cutoff(path, 128)
}
// TODO diagonal connections
pub fn load_graph_from_img_with_cutoff(path: PathBuf, cutoff: u8) -> IoResult<(IdGrid, Graph)> {
    let img = load_luma8(path)?;
    let mut graph = Graph::with_capacity(img.width() as usize * img.height() as usize);

    // TODO use some sort of sparse grid maybe
    let grid = grid_from_img(&img, cutoff);
    let mut id_grid = IdGrid::init(img.width() as usize, img.height() as usize);
    for (y, col) in grid.iter().enumerate() {
        for (x, val) in col.iter().enumerate() {
            if *val {
                let id = Some(graph.add_vertex(Node::new(x as f64, y as f64)));
                if let Some(id) = id {
                    if x > 0 {
                        if let Some(nb) = id_grid.get_2d((x - 1, y)) {
                            graph.add_edge(&id, nb).unwrap();
                        }
                    }
                    if y > 0 {
                        if let Some(nb) = id_grid.get_2d((x, y - 1)) {
                            graph.add_edge(&id, nb).unwrap();
                        }
                    }
                }
                *id_grid.get_mut_2d((x, y)) = id;
            }
        }
    }

    graph.shrink_to_fit();
    Ok((id_grid, graph))
}
