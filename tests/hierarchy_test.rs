use agi_core::Core;

#[test]
fn test_conceptual_hierarchy_learning() {
    println!("\n[INFO] Starting conceptual hierarchy test...");
    let mut core = Core::new();
    
    // Teach relationships
    println!("\n--- Learning Relationships ---");
    core.learn_relationship("Poodle", "Dog");
    core.learn_relationship("Beagle", "Dog");
    core.learn_relationship("Dog", "Canid");
    core.learn_relationship("Wolf", "Canid");
    core.learn_relationship("Canid", "Animal");
    core.learn_relationship("Cat", "Animal");
    core.learn_relationship("Lion", "Cat");
    println!("--- Learning Complete ---");

    // Print the result
    core.conceptual_hierarchy.print_hierarchy();
    println!("[INFO] Conceptual hierarchy test finished.");
}
