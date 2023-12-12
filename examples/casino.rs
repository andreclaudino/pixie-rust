extern crate pixie_rust;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write, Read};

use pixie_rust::recommender::Recommender;

extern crate csv;

fn main() {
    let mut recommender: Recommender<String> = Recommender::new();

    let ratings = load_ratings(&mut recommender);

    let top_recommendations = extract_recommendation(&recommender, &ratings);

    println!("Top Recommendations: {:?}", top_recommendations);
    let output_file_path = "pixie-test.yaml";
    let mut output_file = File::create(output_file_path).unwrap();

    println!("Saving model to disk");
    let serialized_model = recommender.to_yaml().unwrap();
    output_file.write(&serialized_model.as_bytes()).unwrap();
    println!("Model saved to {:?}", output_file_path);

    let other_top_recommendations = extract_recommendation(&recommender, &ratings);
    print!("Other top recommendations: {:?}", other_top_recommendations);

    println!("loading model from disk");
    let mut input_file = File::open(output_file_path).unwrap();
    let mut content = String::new();
    input_file.read_to_string(&mut content).unwrap();
    let new_recommender: Recommender<String> = Recommender::from_yaml(&content).unwrap();
    output_file.write(&serialized_model.as_bytes()).unwrap();
    println!("Model loaded from {:?}", output_file_path);
    
    let new_top_recommendations = extract_recommendation(&new_recommender, &ratings);

    println!("New Recommendations: {:?}", new_top_recommendations);
}

fn load_ratings(recommender: &mut Recommender<String>) -> HashMap<String, f32> {
    println!("Loading Data...");
    // Anime dataset from https://www.kaggle.com/CooperUnion/anime-recommendations-database
    let file = File::open("examples/casino.csv").unwrap();
    let buf_reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(buf_reader);

    let mut ratings: HashMap<String, f32> = HashMap::new();

    for entry_res in csv_reader.records() {
        let entry = entry_res.unwrap();

        let customer_code = entry.get(1).unwrap();
        let categories_str = entry.get(2).unwrap();
        let rating = entry.get(5).unwrap().parse::<f32>().unwrap_or(0.0);
        ratings.insert(String::from(name), rating);
        recommender.add_object(&String::from(name));
        let categories = categories_str.split(",");
        for cat in categories {
            let trimmed = cat.trim();
            recommender.add_tag(trimmed);
            recommender.tag_object(&String::from(name), trimmed);
        }
    }
    println!("Data Loaded!");
    ratings
}

fn extract_recommendation(recommender: &Recommender<String>, ratings: &HashMap<String, f32>) -> Vec<String> {
    let new_top_recommendations = recommender
        .object_recommendations(
            &vec![
                String::from("Cowboy Bebop"),
                String::from("Serial Experiments Lain"),
                String::from("Ghost in the Shell"),
            ],
            30,
            10000,
            |_, _| 1.0,
            |_, to| match to {
                name => ratings.get(name).unwrap_or(&0.0).clone(),
            },
        )
        .iter()
        .take(10)
        .cloned()
        .collect::<Vec<String>>();
    
    new_top_recommendations
}
