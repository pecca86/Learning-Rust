fn main() {
    //let my_ints: Vec<i32> = Vec::new();
    let mut my_ints = vec![1, 5, 3, 2, 7, 6, 4, 5];

    let average_val = count_average(&my_ints);
    println!("Average value: {}", average_val);

    // Sort the list and pick middle value for median
    my_ints.sort_unstable(); // in place sort
    let a_size = my_ints.len();
    let middle_index = a_size / 2;
    println!("Median value: {:?}", &my_ints[middle_index]);

    // Count most recurring value
    use std::collections::HashMap;

    let mut high_fre = HashMap::new();

    for num in my_ints {
        let count = high_fre.entry(num).or_insert(0);
        *count += 1;
    }

    let mut highest: i32 = 0;
    let mut frequency: i32 = 0;

    for (key, value) in high_fre {
        if value > highest {
            highest = key;
            frequency = value;
        }
    }

    println!("This value {} occured the most times ({})", highest, frequency);

    // STRINGS RELATED
    // Pig latin = first consonant is moved to end of word and then we add ay: first = irstfay
    // words that begin with vocal we append 'hay': apple = applehay
    let plain_text = "Pekka var en bondedräng iala iala hej";
    let mut word_list = plain_text.split(" ");
    //let vocals = vec!['a','e','i','o','u'];
    let vocals = "aeiouåäö";

    let mut pig_latin = String::new();

    // Loop trough all the words
    // set them to lowercase and check is they are vocals or consonants
    for mut word in word_list {
        let first_letter = word[0..1].to_ascii_lowercase();
        if vocals.contains(&first_letter) {
            // edel = edelhay
            let mut new_word = word.to_owned() + &"-hay ";
            pig_latin.push_str(&new_word.to_string());
        } else {
            // first = irstfay
            let strlen = word.len();
            let first_part = &word[1..strlen];
            let last_part = &word[0..1];
            let new_word = first_part.to_owned() + &last_part + &"-ay ";
            pig_latin.push_str(&new_word.to_string());
        }
    }

    println!("Final sentence: {}", pig_latin);


    // TASK 3.
    let mut comp: HashMap<String, Vec<String>> = HashMap::new();
    comp.insert(String::from("homot"), vec!["olle".to_string(), "bollle".to_string()]);
    println!("{:?}", comp);


}

// Count average
fn count_average(nums: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;

    for num in &*nums {
        sum += num;
    }

    let elements: i32 = nums.len().try_into().unwrap();
    let average: f32 = (sum / elements) as f32;
    average
}
