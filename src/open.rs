use crate::{csvtype::{CSVType, Byte}, utils::match_catch};
use std::{fs::File, path::Path, io::BufReader};
use rayon::prelude::*;
use std::sync::mpsc::channel;
use itertools::Itertools;

pub fn open_csv(path: &str) -> Result<Vec<Vec<CSVType>>, csv::Error> {
    let mut data = match csv_read(path) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    let (sender, receiver) = channel();
    data.par_iter_mut().enumerate().for_each_with(sender, |s, (i, item)| {
        let mut inner_vec: Vec<CSVType> = Vec::new();
        for (_x, bytes) in item.iter().enumerate() {
            match bytes {
                [] => inner_vec.push(CSVType::Empty),
                _ => {
                    let byte = Byte{b: bytes};
                    match byte {
                        byte if byte.is_number() => inner_vec.push(byte.num_match()),
                        byte if byte.is_dt()=> inner_vec.push(byte.date_and_time()),
                        _ => inner_vec.push(match_catch(bytes)),
                    }
                }
            }
        }
        inner_vec.push(CSVType::Int(i as i64));
        s.send(inner_vec).unwrap();
    });

    let mut res = receiver.iter()
    .sorted_by_key(|x| 
        match x[x.len() - 1] {
            CSVType::Int(v) => v,
            _ => panic!("Not an index somehow"),
        }
    )
    .collect::<Vec<Vec<CSVType>>>();

    for r in 0..res.len() { res[r].pop(); }

    Ok(res)

    // let mut outer_vec: Vec<Vec<CSVType>> = Vec::new();
    // for y in 0..data.len() {
    //     let mut inner_vec: Vec<CSVType> = Vec::new();
    //     for (_x, bytes) in data[y].iter().enumerate() {
    //         match bytes {
    //             [] => inner_vec.push(CSVType::Empty),
    //             _ => {
    //                 let byte = Byte{b: bytes};
    //                 match byte {
    //                     byte if byte.is_number() => inner_vec.push(byte.num_match()),
    //                     byte if byte.is_dt()=> inner_vec.push(byte.date_and_time()),
    //                     _ => inner_vec.push(match_catch(bytes)),
    //                 }
    //             }
    //         }
    //     }
    //     outer_vec.push(inner_vec)
    // }

    // Ok(outer_vec)
}


fn csv_read(path: &str) -> Result<Vec<csv::ByteRecord>, csv::Error> {
    match File::open(Path::new(&path)) {
        Ok(file_handle) => {
            let reader = BufReader::new(file_handle);
            let mut data: Vec<csv::ByteRecord> = Vec::new();
            // Build the CSV reader and iterate over each record.
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(reader);
            for result in rdr.byte_records() {
                match result {
                    Ok(record) => data.push(record),
                    Err(e) => return Err(e),
                };
            }
            return Ok(data)
        },
        Err(e) => return Err(e.into()),
    };
}


// =======================================

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub trait GridLike<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn get(&self, p: Point) -> &T;

    fn set_all_parallel<F>(&mut self, setter: F)
    where 
        F: Send + Sync + Fn(Point) -> T,
        T: Send;
}

pub struct Grid<T> {
    pub array: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
        where 
            T: Default + Copy, {
        Self {
            array: [T::default()].repeat(width * height),
            width,
            height,
        }
    }
}

impl<T> GridLike<T> for Grid<T> {
    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
    fn get(&self, p: Point) -> &T { &self.array[p.y * self.width + p.x] }
    fn set_all_parallel<F>(&mut self, setter: F)
        where 
            F: Send + Sync + Fn(Point) -> T,
            T: Send {
                let width = self.width;
        self.array.par_iter_mut().enumerate().for_each(|(i, item)| {
            *item = setter(Point { 
                x: i % width,
                y: i / width,
            });
        })
    }
}