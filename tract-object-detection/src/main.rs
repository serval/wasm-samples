use std::io::prelude::*;
use std::io::BufReader;

use tract_tensorflow::prelude::{tract_itertools::Itertools, *};
use tract_tensorflow::tract_hir::internal::anyhow;

fn main() -> TractResult<()> {
    // load the labels
    let labels_str = include_str!("../imagenet_slim_labels.txt");
    let labels: Vec<_> = labels_str.lines().collect();

    let mut model_data: &[u8] = include_bytes!("../models/mobilenet_v2_1.4_224_frozen.pb");
    let model = tract_tensorflow::tensorflow()
        // load the model
        .model_for_read(&mut model_data)?
        // specify input type and shape
        .with_input_fact(0, f32::fact([1, 224, 224, 3]).into())?
        // optimize the model
        .into_optimized()?
        // make the model runnable and fix its inputs and outputs
        .into_runnable()?;

    // open image, resize it and make a Tensor out of it
    if atty::is(atty::Stream::Stdin) {
        return Err(anyhow!("Provide image data on stdin"));
    }

    let mut image_data: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(std::io::stdin());
    let bytes_read = reader.read_to_end(&mut image_data)?;
    if bytes_read == 0 {
        return Err(anyhow!("Provide image data on stdin"));
    }

    let image = image::load_from_memory(&image_data)?;
    let resized =
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    })
    .into();

    // run the model on the input
    let result = model.run(tvec!(image.into()))?;

    // take the top N results and display them, along with their label
    const MAX_RESULTS: usize = 10;
    const MIN_CONFIDENCE: f32 = 0.05;

    let sorted = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .rev()
        .take(MAX_RESULTS)
        .filter(|(confidence, _)| *confidence >= MIN_CONFIDENCE)
        .map(|(confidence, label_idx)| (confidence, label_idx, labels[label_idx - 1]));

    for (confidence, _, label) in sorted {
        println!("{label} ({:.2}%)", (confidence * 100.0));
    }
    // let best = sorted.last();
    // println!("result: {:?}", best);
    Ok(())
}
