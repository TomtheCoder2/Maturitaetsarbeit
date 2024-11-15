use image::GenericImageView;
use tensorflow::{Graph, SavedModelBundle, SessionOptions, SessionRunArgs, Tensor};
fn main() {
    // In this file test_in_input is being used while in the python script,
    // that generates the saved model from Keras model it has a name "test_in".
    // For multiple inputs _input is not being appended to signature input parameter name.
    let signature_input_parameter_name = "test_in_input";
    let signature_output_parameter_name = "test_out";

    // Initialize save_dir, input tensor, and an empty graph
    let save_dir = "../../python_code_ma/detect_player/";
    // load image player_training_images/34_32_2_2_0010.png
    let img = image::open("../../player_training_images/34_32_2_2_0010.png").unwrap();
    let img = img.resize_exact(128, 128, image::imageops::FilterType::Nearest);
    println!(
        "length: {}, dimensions: {:?}",
        img.to_rgb8().into_raw().len(),
        img.dimensions()
    );
    let value = img
        .to_rgb8()
        .into_raw()
        .iter()
        .map(|&v| v as f32)
        .collect::<Vec<f32>>();
    let tensor: Tensor<f32> = Tensor::new(&[128, 128, 3])
        .with_values(&value)
        .expect("Can't create tensor");
    let mut graph = Graph::new();

    // Load saved model bundle (session state + meta_graph data)
    let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, save_dir)
        .expect("Can't load saved model");

    // Get the session from the loaded model bundle
    let session = &bundle.session;

    // Get signature metadata from the model bundle
    let signature = bundle
        .meta_graph_def()
        .get_signature("serving_default")
        .unwrap();

    signature.outputs().iter().for_each(|(name, info)| {
        println!("Input: {:?}, {:?}", name, info);
    });

    // Get input/output info
    let input_info = signature.get_input("inputs").unwrap();
    let output_info = signature.get_output("output_0").unwrap();

    // Get input/output ops from graph
    let input_op = graph
        .operation_by_name_required(&input_info.name().name)
        .unwrap();
    let output_op = graph
        .operation_by_name_required(&output_info.name().name)
        .unwrap();

    // Manages inputs and outputs for the execution of the graph
    let mut args = SessionRunArgs::new();
    args.add_feed(&input_op, 0, &tensor); // Add any inputs

    let out = args.request_fetch(&output_op, 0); // Request outputs

    //  session = keras.backend.get_session()
    // init = tf.global_variables_initializer()
    // session.run(init)

    // Run model
    session
        .run(&mut args) // Pass to session to run
        .expect("Error occurred during calculations");

    // Fetch outputs after graph execution
    let out_res: f32 = args.fetch(out).unwrap()[0];

    println!("Results: {:?}", out_res);
}
