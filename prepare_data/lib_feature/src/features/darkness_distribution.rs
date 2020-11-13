use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::{Feature, MaximumValue};

pub fn calc_darkness_distribution_float_x(length: usize, gesture: &Gesture) -> Vec<f64> {
    assert!(gesture.frames.len() >= length);
    let mut result = Vec::new();
    let max_frame = MaximumValue::calculate(gesture);
    for frame in gesture.frames.iter() {
        let row1 = max_frame.deref()[0] + max_frame.deref()[3] + max_frame.deref()[6] - frame.pixel[0] - frame.pixel[3] - frame.pixel[6];
        let row2 = max_frame.deref()[1] + max_frame.deref()[4] + max_frame.deref()[7] - frame.pixel[1] - frame.pixel[4] - frame.pixel[7];
        let row3 = max_frame.deref()[2] + max_frame.deref()[5] + max_frame.deref()[8] - frame.pixel[2] - frame.pixel[5] - frame.pixel[8];
        result.push(vec![row1, row2, row3].into_iter().enumerate()
            .max_by(|&(_, left), &(_, right)| left.cmp(&right)).unwrap().0);
    }
    let merge_threshold = result.len() as f64 / (length as f64);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f64) < merge_threshold * ((perma_result.len() + 1) as f64) {
            continue;
        }
        perma_result.push((values.iter().sum::<usize>() as f64) / (values.len() as f64));
        values.clear();
    }
    perma_result
}

pub fn calc_darkness_distribution_float_y(length: usize, gesture: &Gesture) -> Vec<f64> {
    assert!(gesture.frames.len() >= length);
    let mut result = Vec::new();
    let max_frame = MaximumValue::calculate(gesture);
    for frame in gesture.frames.iter() {
        let row1 = max_frame.deref()[0] + max_frame.deref()[1] + max_frame.deref()[2] - frame.pixel[0] - frame.pixel[1] - frame.pixel[2];
        let row2 = max_frame.deref()[3] + max_frame.deref()[4] + max_frame.deref()[5] - frame.pixel[3] - frame.pixel[4] - frame.pixel[5];
        let row3 = max_frame.deref()[6] + max_frame.deref()[7] + max_frame.deref()[8] - frame.pixel[6] - frame.pixel[7] - frame.pixel[8];
        result.push(vec![row1, row2, row3].into_iter().enumerate()
            .max_by(|&(_, left), &(_, right)| left.cmp(&right)).unwrap().0);
    }
    let merge_threshold = result.len() as f64 / (length as f64);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f64) < merge_threshold * ((perma_result.len() + 1) as f64) {
            continue;
        }
        perma_result.push((values.iter().sum::<usize>() as f64) / (values.len() as f64));
        values.clear();
    }
    perma_result
}

pub fn calc_darkness_distribution_float_xy(length: usize, gesture: &Gesture) -> Vec<usize> {
    assert!(gesture.frames.len() >= length);
    let mut result = Vec::new();
    let max_frame = MaximumValue::calculate(gesture);
    for frame in gesture.frames.iter() {
        result.push(frame.pixel.iter().enumerate().map(|(index, item)| (index, max_frame.deref()[index] - item))
            .max_by(|&(_, left), &(_, right)| left.cmp(&right)).unwrap().0)
    }
    let merge_threshold = result.len() as f64 / (length as f64);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f64) < merge_threshold * ((perma_result.len() + 1) as f64) {
            continue;
        }
        perma_result.push(values.iter().max().cloned().unwrap());
        values.clear();
    }
    perma_result
}

pub fn calc_darkness_distribution_float_xy_geom(length: usize, gesture: &Gesture) -> Vec<usize> {
    assert!(gesture.frames.len() >= length);
    let mut result = Vec::new();
    let max_frame = MaximumValue::calculate(gesture);
    for frame in gesture.frames.iter() {
        let min_index = frame.pixel.iter().enumerate().map(|(index, item)| (index, max_frame.deref()[index] - item))
            .max_by(|&(_, left), &(_, right)| left.cmp(&right)).unwrap().0;
        result.push(match min_index {
            0 => (0, 2),
            1 => (1, 2),
            2 => (2, 2),
            3 => (0, 1),
            4 => (1, 1),
            5 => (2, 1),
            6 => (0, 0),
            7 => (1, 0),
            8 => (2, 0),
            _ => unreachable!()
        })
    }
    let merge_threshold = result.len() as f64 / (length as f64);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f64) < merge_threshold * ((perma_result.len() + 1) as f64) {
            continue;
        }
        let (x, y) = values.iter().fold((0, 0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));
        perma_result.push(match (x / values.len(), y / values.len()) {
            (0, 0) => 6,
            (1, 0) => 7,
            (2, 0) => 8,
            (0, 1) => 3,
            (1, 1) => 4,
            (2, 1) => 5,
            (0, 2) => 0,
            (1, 2) => 1,
            (2, 2) => 2,
            _ => unreachable!()
        });
        values.clear();
    }
    perma_result
}

pub fn calc_darkness_distribution_float_xy_quadrant(length: usize, gesture: &Gesture) -> Vec<usize> {
    assert!(gesture.frames.len() >= length);
    let mut result = Vec::new();
    let max_frame = MaximumValue::calculate(gesture);
    for frame in gesture.frames.iter() {
        let quadrant1 = max_frame.deref()[0] + max_frame.deref()[1] + max_frame.deref()[3] + max_frame.deref()[4] - frame.pixel[0] - frame.pixel[1] - frame.pixel[3] - frame.pixel[4];
        let quadrant2 = max_frame.deref()[1] + max_frame.deref()[2] + max_frame.deref()[4] + max_frame.deref()[5] - frame.pixel[1] - frame.pixel[2] - frame.pixel[4] - frame.pixel[5];
        let quadrant3 = max_frame.deref()[3] + max_frame.deref()[4] + max_frame.deref()[6] + max_frame.deref()[7] - frame.pixel[3] - frame.pixel[4] - frame.pixel[6] - frame.pixel[7];
        let quadrant4 = max_frame.deref()[4] + max_frame.deref()[5] + max_frame.deref()[7] + max_frame.deref()[8] - frame.pixel[4] - frame.pixel[5] - frame.pixel[7] - frame.pixel[8];

        result.push(match vec![quadrant1, quadrant2, quadrant3, quadrant4].iter().enumerate()
            .max_by(|&(_, left), &(_, right)| left.cmp(right)).unwrap().0 {
            0 => (0, 1),
            1 => (1, 1),
            2 => (0, 0),
            3 => (1, 0),
            _ => unreachable!()
        })
    }
    let merge_threshold = result.len() as f64 / (length as f64);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f64) < merge_threshold * ((perma_result.len() + 1) as f64) {
            continue;
        }
        let (x, y) = values.iter().fold((0, 0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));
        perma_result.push(match (x / values.len(), y / values.len()) {
            (0, 1) => 0,
            (1, 1) => 1,
            (0, 0) => 2,
            (1, 0) => 3,
            _ => 4
        });
        values.clear();
    }
    perma_result
}