use std::ops::Deref;

use lib_gesture::entities::Gesture;

use crate::features::{Feature, MaximumValue};

/// In X direction, find for a specified length of timeframes the area (left 0, center 1, right 2)
/// where it is darkest for each time frame, i.e. where the difference to the maximum is the maximal (Maximum - current).
/// Its called float because it can also be also take values between the areas.
pub fn calc_darkness_distribution_float_x(length: usize, gesture: &Gesture) -> Vec<f32> {
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

    // Tries to shorten to result to the specified length
    let merge_threshold = result.len() as f32 / (length as f32);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f32) < merge_threshold * ((perma_result.len() + 1) as f32) {
            continue;
        }
        perma_result.push((values.iter().sum::<usize>() as f32) / (values.len() as f32));
        values.clear();
    }
    perma_result
}

/// In Y direction, find for a specified length of timeframes the area (top 0, center 1, bottom 2)
/// where it is darkest for each time frame, i.e. where the difference to the maximum is the maximal (Maximum - current).
/// Its called float because it can also be also take values between the areas.
pub fn calc_darkness_distribution_float_y(length: usize, gesture: &Gesture) -> Vec<f32> {
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

    // Tries to shorten to result to the specified length
    let merge_threshold = result.len() as f32 / (length as f32);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f32) < merge_threshold * ((perma_result.len() + 1) as f32) {
            continue;
        }
        perma_result.push((values.iter().sum::<usize>() as f32) / (values.len() as f32));
        values.clear();
    }
    perma_result
}

/// In XY direction, find for a specified length of timeframes the area (top 0,1,2, center 3,4,5, bottom 6,7,8)
/// where it is darkest for each time frame, i.e. where the difference to the maximum is the maximal (Maximum - current).
/// Its called float because it can also be also take values between the areas.
pub fn calc_darkness_distribution_float_xy(length: usize, gesture: &Gesture) -> Vec<usize> {
    assert!(gesture.frames.len() >= length);
    let mut result = Vec::new();
    let max_frame = MaximumValue::calculate(gesture);
    for frame in gesture.frames.iter() {
        result.push(frame.pixel.iter().enumerate().map(|(index, item)| (index, max_frame.deref()[index] - item))
            .max_by(|&(_, left), &(_, right)| left.cmp(&right)).unwrap().0)
    }

    // Tries to shorten to result to the specified length
    let merge_threshold = result.len() as f32 / (length as f32);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f32) < merge_threshold * ((perma_result.len() + 1) as f32) {
            continue;
        }
        perma_result.push(values.iter().max().cloned().unwrap());
        values.clear();
    }
    perma_result
}

/// In XY direction, find for a specified length of timeframes the area (top 0,1,2, center 3,4,5, bottom 6,7,8)
/// where it is darkest for each time frame, i.e. where the difference to the maximum is the maximal (Maximum - current).
/// In order to squish it better into the specified length, it maps the 9 areas to a cartesian system and uses the average of the sum of the coordinates.
/// Its called float because it can also be also take values between the areas.
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

    // Tries to shorten to result to the specified length
    let merge_threshold = result.len() as f32 / (length as f32);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f32) < merge_threshold * ((perma_result.len() + 1) as f32) {
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

/// In XY direction, find for a specified length of timeframes the area (top left (0,1,3,4), top right (1,2,4,5), bottom left (3,4,6,7), bottom right (4,5,7,8), center (1,3,4,5,7))
/// where it is darkest for each time frame, i.e. where the difference to the maximum is the maximal (Maximum - current).
/// It first maps all all indices geometrically to the first 4 quadrants and then sums them. Those that dont fit in one of the 4 first quadrants specifically a categorized as center.
/// Its called float because it can also be also take values between the areas.
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
    let merge_threshold = result.len() as f32 / (length as f32);
    let mut values = Vec::new();
    let mut perma_result = Vec::with_capacity(length);
    for i in 0..result.len() {
        values.push(result[i]);
        if ((i + 1) as f32) < merge_threshold * ((perma_result.len() + 1) as f32) {
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