use std::fs::File;
use std::io::{self, BufRead};

pub fn get_dataset() -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let file = File::open("/data/oc-compute03/trautwju/Masterarbeit/src/datasets/Data/Regression/bike_sharing_day.csv").expect("Couldn't open input");
    let mut csv_file = csv::Reader::from_reader(file);
    let data_len = csv_file.records().count();
    let mut data = vec![vec![0.; 14]; data_len];
    let mut labels = vec![vec![0.; 1]; data_len];
    let mut standardisation_labels:Vec<f32> = vec![0.; data_len + 1];
    let mut mins = vec![100000000000.; 14];
    let mut maxs = vec![-10000000000.; 14];
    let mut i = 0;
    let file = File::open("/data/oc-compute03/trautwju/Masterarbeit/src/datasets/Data/Regression/bike_sharing_day.csv").expect("Couldn't open input");
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let record = line.unwrap();
        let vector:Vec<&str> = record.split(",").collect();
        if vector[0] == "instant"{
            continue;
        }
        data[i][0] = vector[0].trim().parse::<f32>().unwrap();
        for j in 2..15{
            data[i][j-1] = vector[j].trim().parse::<f32>().unwrap();
        }
        labels[i][0] = vector[15].trim().parse::<f32>().unwrap();
        standardisation_labels[i] = labels[i][0];
        for j in 0..14{
            if data[i][j] > maxs[j]{
                maxs[j] = data[i][j]
            }
            if data[i][j] < mins[j]{
                mins[j] = data[i][j]
            }
        }
        i += 1;
    }

    let mean:f32 = standardisation_labels.iter().sum::<f32>() / standardisation_labels.len() as f32;
    let mut deviation:f32 = standardisation_labels
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f32>();
    deviation /= standardisation_labels.len() as f32;
    deviation = deviation.sqrt();

    for i in 0..data.len(){
        for j in 0..data[0].len(){
            data[i][j] = (data[i][j] - mins[j]) / (maxs[j] - mins[j])
        }
        labels[i][0] = (labels[i][0] - mean) / deviation;
    }

    return (data, labels);
}