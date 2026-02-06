use agi_core::conceptual_hierarchy::ConceptualHierarchy;
use agi_core::trace_visualizer::generate_trace_image;

#[test]
fn test_hierarchy_save_and_load() {
    // 1. Create and populate the original hierarchy
    let mut original_hierarchy = ConceptualHierarchy::new();
    original_hierarchy.learn_relationship_by_name("Poodle", "Dog");
    original_hierarchy.learn_relationship_by_name("Beagle", "Dog");
    original_hierarchy.learn_relationship_by_name("Dog", "Canid");
    original_hierarchy.learn_relationship_by_name("Wolf", "Canid");
    original_hierarchy.learn_relationship_by_name("Canid", "Animal");
    original_hierarchy.learn_relationship_by_name("Cat", "Animal");
    original_hierarchy.learn_relationship_by_name("Lion", "Cat");

    let file_path = "test_hierarchy.hl";

    // 2. Save the hierarchy to file
    original_hierarchy.save_to_file(file_path).unwrap();

    // 3. Load the hierarchy from file
    let loaded_hierarchy = ConceptualHierarchy::load_from_file(file_path).unwrap();

    // 4. Assert that the loaded hierarchy is identical to the original
    assert_eq!(original_hierarchy, loaded_hierarchy);

    // 5. Generate and save a visualization of a concept's trace
    if let Some(dog_concept) = loaded_hierarchy.find_concept_by_name("Dog") {
        let trace_image = generate_trace_image(&dog_concept.trace, 512, 512);
        let image_path = "dog_trace.png";
        let save_result = trace_image.save(image_path);
        assert!(save_result.is_ok(), "Failed to save trace image.");
        println!("--- Generated trace image for 'Dog' concept at {} ---", image_path);
    } else {
        panic!("Could not find 'Dog' concept to visualize.");
    }

    // 6. Clean up the test file
    let _ = std::fs::remove_file(file_path);
}
