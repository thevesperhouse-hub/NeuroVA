// agi_core: The scientific and logical heart of the Biomimetic AGI.
// This crate will contain no graphics-related code.
// It will implement the core data structures and algorithms for:
// - Fractal compression
// - Holographic memory
// - Neuro-symbolic reasoning

pub mod neuron;

pub mod connectome;
pub mod conceptual_hierarchy;
pub mod quantum;
pub mod thalamus;
pub mod hippocampus;
pub mod quantum_gatekeeper;
pub mod reasoning_engine;
pub mod creativity_forge;
pub mod sensory_cortex;
pub mod synthesis_cortex;
pub mod performance_monitor;

pub mod motor_cortex;
pub mod knowledge_explorer;
pub mod self_awareness;
pub mod silicium;
pub mod holographic_memory;
pub mod lemmatizer;
pub mod curiosity_engine;
pub mod knowledge_scanner;
pub mod prefrontal_cortex;
pub mod ethical_core;
pub mod synthesis;
pub mod social_cortex;
pub mod prompt_segmenter;
pub mod mcq_solver;
pub mod direct_answer_extractor;
pub mod personality;
pub mod inner_drive;
pub mod neurochemical_modulator;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Helper function to read a file line by line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use connectome::Connectome;
use std::sync::{Arc, Mutex, RwLock, atomic::Ordering};
use std::time::Instant;
use atomic_float::AtomicF32;

pub use quantum::{Qubit, HadamardGate, OneQubitGate};
use thalamus::{QueryType, Thalamus};
use hippocampus::Hippocampus;
use quantum_gatekeeper::QuantumGatekeeper;
use reasoning_engine::ReasoningEngine;
use creativity_forge::CreativityForge;
use sensory_cortex::SensoryCortex;
use crate::motor_cortex::MotorCortex;
use crate::self_awareness::SelfAwareness;
use crate::knowledge_explorer::KnowledgeExplorer;
use crate::holographic_memory::{ConceptFocuser, HolographicEncoder};
use crate::knowledge_scanner::{DataSource, KnowledgeScanner};
use crate::prefrontal_cortex::PrefrontalCortex;
use crate::ethical_core::EthicalCore;
use crate::conceptual_hierarchy::ConceptualHierarchy;
use crate::social_cortex::SocialCortex;
use crate::mcq_solver::McqSolver;
use crate::inner_drive::InnerDrive;

use crate::neurochemical_modulator::NeurochemicalModulator;


use crate::holographic_memory::HolographicMemory;


pub struct Core {
    mcq_solver: Option<McqSolver>,

    pub tick: u64,
    pub connectome: Connectome,
    pub quantum_core: Vec<Qubit>,
    pub thalamus: Thalamus,
    pub hippocampus: Hippocampus,
    pub gatekeeper: QuantumGatekeeper,
    pub reasoning_engine: Arc<Mutex<ReasoningEngine>>,
    pub creativity_forge: CreativityForge,
    pub prefrontal_cortex: PrefrontalCortex,
    pub ethical_core: EthicalCore,
    pub self_awareness: SelfAwareness,
    pub sensory_cortex: SensoryCortex,
    pub motor_cortex: MotorCortex,
    pub knowledge_explorer: KnowledgeExplorer,
    pub knowledge_scanner: KnowledgeScanner,
    pub conceptual_hierarchy: ConceptualHierarchy,
        pub social_cortex: SocialCortex,
    pub neurochemical_modulator: NeurochemicalModulator,
    pub direct_answer_extractor: direct_answer_extractor::DirectAnswerExtractor,
    pub inner_drive: InnerDrive,

    pub holographic_encoder: Arc<RwLock<HolographicEncoder>>,
    quantum_state_initialized: bool,
    pub firing_rate: f32,
    wakeup_stages: u32,
    current_wakeup_stage: u32,
    pub response_pending: bool,

    last_response: Arc<Mutex<Option<String>>>,
    pub last_reasoning_result: Option<String>, // Stores the text of the last successful reasoning result.
    pub last_fired_neurons: Vec<u64>,

    // Performance metrics
    pub processing_speed: Arc<AtomicF32>,
    pub power_draw: Arc<AtomicF32>,
    energy_this_measurement_period: f32,
    last_measurement_time: Instant,
    ticks_this_measurement_period: u64,
}

impl Core {
    /// Assimilates a piece of text into the AGI's consciousness, with an option to treat it as a foundational axiom.
    pub fn learn_and_assimilate(&mut self, text: &str, is_axiom: bool) {
        // 1. Translate text into a list of neural stimuli.
        let stimuli = self.sensory_cortex.process_text(text, &mut self.conceptual_hierarchy, &self.holographic_encoder.read().unwrap());

        // 2. Apply these stimuli to the connectome.
        for (neuron_id, strength) in stimuli {
            if let Some(neuron) = self.connectome.neurons.get_mut(neuron_id as usize) {
                // For axioms, we give an even bigger initial boost to ensure they fire strongly.
                let boost = if is_axiom { strength * 1.5 } else { strength };
                neuron.potential += boost;
            }
        }

        // 3. Force an immediate update to identify which neurons fired in response to the stimulus.
        let active_ids_vec = self.connectome.update(self.tick);
        let active_ids_set: std::collections::HashSet<u64> = active_ids_vec.into_iter().collect();

        // 4. Apply potentiation. Deeply engrave axioms, apply standard LTP for regular knowledge.
        if is_axiom {
            self.connectome.deeply_engrave_pathway(&active_ids_set);
        } else {
            self.connectome.potentiate_pathway(&active_ids_set);
        }

        // 5. Kick-start the resonance by propagating the initial signal immediately.
        for &neuron_id in &active_ids_set {
            self.connectome.propagate_signal_from(neuron_id);
        }

        // 6. Now, encode the resulting neural activity pattern into a holographic trace.
        let trace = self.holographic_encoder.read().unwrap().encode(text);

        // 7. Store this new trace in the hippocampus as a permanent memory.
        self.hippocampus.add_holographic_memory(text.to_string(), trace, is_axiom);
    }



    /// Apprend à partir d'une source de données externe en la scannant.
    ///
    /// Cette méthode utilise le KnowledgeScanner pour extraire une signature informationnelle
    /// d'une source (comme une URL ou un fichier local) sans la télécharger entièrement.
    /// La signature est ensuite traitée comme un souvenir unique et encodée holographiquement.
    pub async fn learn_from_source(&mut self, source: &DataSource) {
        println!("--- Début de l'apprentissage par scan de source : {:?} ---", source);
        const NUM_FRAGMENTS: u32 = 20; // Nombre de fragments à extraire
        const FRAGMENT_SIZE: u64 = 2048; // Taille de chaque fragment en octets

        match self.knowledge_scanner.scan(source, NUM_FRAGMENTS, FRAGMENT_SIZE).await {
            Ok(signature) => {
                println!("Scan réussi. Signature de {} octets générée. Début de l'encodage holographique.", signature.len());
                // Nous utilisons la méthode d'apprentissage existante pour encoder la signature.
                self.learn_and_assimilate(&signature, false);
                println!("--- Apprentissage par scan terminé avec succès. ---");
            }
            Err(e) => {
                eprintln!("Erreur lors du scan de la source de connaissances: {}", e);
                println!("--- Apprentissage par scan échoué. ---");
            }
        }
    }

    const HOLOGRAPHIC_DIMENSION: usize = 1024;

pub fn new(_knowledge_file_path: Option<&str>) -> Self {
        let concept_focuser = ConceptFocuser::new();
        // Load the connectome from the binary file.
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let workspace_root = Path::new(manifest_dir).parent().unwrap();
        let connectome_path = workspace_root.join("quantized_connectome.bin");

        let connectome = Connectome::from_binary(&connectome_path)
            .unwrap_or_else(|e| {
                panic!("Failed to load connectome from {:?}. Did you run the 'gen_connectome' tool? Error: {}", connectome_path, e)
            });

        // Initialize the Quantum Core with a set of qubits
        let num_qubits = Self::HOLOGRAPHIC_DIMENSION;
        let mut quantum_core = (0..num_qubits).map(|_| Qubit::new()).collect::<Vec<_>>();
        let hippocampus = Hippocampus::new();

        // Prime the AGI with core memories at boot.
        hippocampus.replay_core_memories(&mut quantum_core);

        let personality = personality::Personality::new();
        let motor_cortex = MotorCortex::new(personality);
        let reasoning_engine = Arc::new(Mutex::new(ReasoningEngine::new()));
        let creativity_forge = CreativityForge::new();
        let self_awareness = SelfAwareness::new("identity.txt", &hippocampus);
        let inner_drive = InnerDrive::new(5); // Autonomous thoughts every 5 seconds.

        let holographic_encoder = Arc::new(RwLock::new(HolographicEncoder::new(Self::HOLOGRAPHIC_DIMENSION)));

        let mut new_core = Self {
            last_reasoning_result: None,
            mcq_solver: None, // Initialized to None, will be set later.

            tick: 0,
            connectome,
            quantum_core,
            thalamus: Thalamus::new(Arc::clone(&holographic_encoder)),
            hippocampus,
            gatekeeper: QuantumGatekeeper::new(),
            reasoning_engine: Arc::clone(&reasoning_engine),
            prefrontal_cortex: PrefrontalCortex::new(concept_focuser.clone()),
            ethical_core: EthicalCore::new(),
            creativity_forge,
            self_awareness,
            sensory_cortex: SensoryCortex::new(),
            motor_cortex,
            knowledge_explorer: KnowledgeExplorer::new(),
            knowledge_scanner: KnowledgeScanner::new(),
            conceptual_hierarchy: ConceptualHierarchy::new(),
                        social_cortex: SocialCortex::new(),
            neurochemical_modulator: NeurochemicalModulator::new(),
            direct_answer_extractor: direct_answer_extractor::DirectAnswerExtractor::new(),
            inner_drive,

            holographic_encoder,
            quantum_state_initialized: false,
            firing_rate: 0.0,
            wakeup_stages: 0,
            current_wakeup_stage: 0,
            response_pending: false,
            last_response: Arc::new(Mutex::new(None)),
            last_fired_neurons: Vec::new(),
            processing_speed: Arc::new(AtomicF32::new(0.0)),
            power_draw: Arc::new(AtomicF32::new(0.0)),
            energy_this_measurement_period: 0.0,
            last_measurement_time: Instant::now(),
            ticks_this_measurement_period: 0,
        };

        // --- The Awakening Ritual: Assimilating the Foundational Corpus ---
        println!("\n--- The Awakening Ritual has begun. Assimilating foundational wisdom. ---");
        let corpus_dir = workspace_root.join("corpus_fondamental");
        if corpus_dir.is_dir() {
            match std::fs::read_dir(corpus_dir) {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                println!("--- Reading from wisdom file: {:?} ---", path.file_name().unwrap_or_default());
                                if let Ok(content) = std::fs::read_to_string(&path) {
                                    for line in content.lines() {
                                        if !line.trim().is_empty() {
                                            new_core.learn_and_assimilate(line, true);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Warning: Could not read corpus_fondamental directory: {}. AGI will lack foundational wisdom.", e),
            }
        } else {
            eprintln!("Warning: 'corpus_fondamental' directory not found. AGI will lack foundational wisdom.");
        }

        let knowledge_path = workspace_root.join("knowledge.txt");
        if knowledge_path.exists() {
            println!("--- The Awakening Ritual: Assimilating foundational knowledge from knowledge.txt... ---");
            if let Ok(lines) = read_lines(&knowledge_path) {
                let mut lines_iter = lines.filter_map(Result::ok).peekable();
                while let Some(line) = lines_iter.next() {
                    let fact_text = line.trim();
                    if fact_text.is_empty() || fact_text.starts_with("//") {
                        continue;
                    }

                    // 1. Assimilate the fact.
                    new_core.learn_and_assimilate(fact_text, true);
                    
                    // Attempt to get the concept ID using the correct method.
                    if let Some(fact_concept) = new_core.conceptual_hierarchy.find_concept_by_name(fact_text) {
                        let fact_concept_id = fact_concept.id;
                        // 2. Check if the next line is a domain comment.
                        if let Some(true) = lines_iter.peek().map(|line| line.trim().starts_with("// domains:")) {
                            // It is a domain comment, so we can safely consume it.
                            if let Some(comment_line) = lines_iter.next() {
                                let domains_str = comment_line.trim().replace("// domains:", "").trim().to_string();
                                let domain_names: Vec<&str> = domains_str.split(',').map(|s| s.trim()).collect();

                                for domain_name in domain_names {
                                    if domain_name.is_empty() { continue; }
                                    let domain_id = new_core.conceptual_hierarchy.find_or_create_concept(domain_name);
                                    new_core.conceptual_hierarchy.add_domain_to_concept(fact_concept_id, domain_id);
                                    println!("    -> Linked concept '{}' to domain '{}'", fact_text, domain_name);
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("--- The Awakening Ritual is complete. ---");

        // Now that all foundational memories are loaded, build the document frequency map for TF-IDF.
        new_core.holographic_encoder.write().unwrap().build_document_frequency(&new_core.hippocampus.holographic_memory);

        // Rebuild Thalamus prototypes with the mature encoder.
        new_core.thalamus.rebuild_prototypes();

        // Finally, create the MCQ solver with the fully initialized reasoning engine.
        new_core.mcq_solver = Some(McqSolver::new(Arc::clone(&new_core.reasoning_engine)));

        println!("--- AGI Core Initialized ---");

        new_core
    }

    /// Rebuilds the Thalamus prototypes. This should be called after all initial knowledge
    /// has been assimilated to ensure the semantic space is mature.
    pub fn rebuild_thalamus_prototypes(&mut self) {
        self.thalamus.rebuild_prototypes();
    }

    pub fn tick(&mut self) -> Option<String> {
        // --- Neuro-Modulation: Homeostasis ---
        // Simulate the natural decay of neurochemicals over time.
        self.neurochemical_modulator.decay();

        self.tick += 1;

        // --- Inner Drive: Autonomous Thought Generation ---
        // --- Inner Drive désactivé temporairement pour se concentrer sur la qualité de la réponse directe.
        /*
        if let Some(internal_prompt) = self.inner_drive.tick(self.last_reasoning_result.as_deref(), &self.hippocampus.holographic_memory) {
            // An autonomous thought was generated. The AGI will now process it.
            // The result of this internal reasoning becomes the new context for the next Inner Drive tick.
            if let Some((response, _query_type)) = self.get_response_for_prompt(&internal_prompt) {
                self.last_reasoning_result = Some(response);
            }
        }
        */


        // --- Start of Simulation Step ---

        // 1. Initialize Quantum State Superposition (only once)
        if !self.quantum_state_initialized {
            let hadamard_gate = HadamardGate;
            for qubit in self.quantum_core.iter_mut() {
                hadamard_gate.apply(qubit);
            }
            self.quantum_state_initialized = true;
            // println!("--- Quantum Core Superposition Initialized by Thalamus ---");
        }

        // The old reasoning logic has been removed from the tick loop.
        // Reasoning is now handled exclusively in `stimulate_for_prompt`.

        // The artificial stimulation has been removed. The network must now rely on
        // the potentiated pathways for organic recall.

        // 3. Update all neurons in the connectome. This handles potential decay and firing checks.
        let active_neuron_ids = self.connectome.update(self.tick);
        self.last_fired_neurons = active_neuron_ids.clone();

        // --- Update Performance Metrics ---
        // Accumulate energy for this measurement period
        self.energy_this_measurement_period += active_neuron_ids.len() as f32 * 0.00015; // Scaled energy cost per firing

        // Calculate processing speed and power draw once per second
        self.ticks_this_measurement_period += 1;
        let elapsed = self.last_measurement_time.elapsed();
        if elapsed.as_secs_f32() >= 1.0 {
            let elapsed_secs = elapsed.as_secs_f32();
            // Speed is ticks per second
            self.processing_speed.store(self.ticks_this_measurement_period as f32 / elapsed_secs, Ordering::Relaxed);
            // Power is energy per second (Watts)
            self.power_draw.store(self.energy_this_measurement_period / elapsed_secs, Ordering::Relaxed);

            // Reset for next measurement period
            self.last_measurement_time = Instant::now();
            self.ticks_this_measurement_period = 0;
            self.energy_this_measurement_period = 0.0;
        }

        // 4. Propagate signals from firing neurons.
        for &neuron_id in &active_neuron_ids {
            self.connectome.propagate_signal_from(neuron_id);
        }

        // 5. Imprint the current neural activity onto the quantum core.
        for neuron in &self.connectome.neurons {
            if neuron.potential > 0.01 { // Use a small threshold to avoid noise
                if let Some(qubit) = self.quantum_core.get_mut(neuron.id as usize) {
                    // The phase is proportional to the neuron's potential.
                    // The constant factor can be tuned to adjust sensitivity.
                    let phase = neuron.potential * 0.5;
                    let phase_gate = quantum::PhaseShiftGate::new(phase);
                    phase_gate.apply(qubit);
                }
            }
        }

        // 5. Engage cognitive functions.
        // self.reasoning_engine.process(&mut self.quantum_core, &self.hippocampus);
        // self.creativity_forge.process(&mut self.quantum_core);

        // 6. Generate a response if one has been requested.
        // 6. If a response has been generated and is ready, return it.


        None
    }

    pub fn get_response(&mut self) -> Option<String> {
        // Atomically take the response. This guarantees that a response is consumed exactly once.
        self.last_response.lock().unwrap().take()
    }

    /// Clears the last response from the core, to be called by the UI after displaying it.
    pub fn clear_response(&mut self) {
        *self.last_response.lock().unwrap() = None;
    }

    /// The main, modern entry point for processing a prompt and generating a response.
    pub fn get_response_for_prompt(&mut self, prompt: &str) -> Option<(String, QueryType)> {
        // --- Step 0: Update Conversational Context --- 
        self.prefrontal_cortex.update_context(prompt);

        // --- Step 1: Ethical Gatekeeping (Input Validation) ---
        if let crate::ethical_core::EthicalJudgment::Reject(reason) = self.ethical_core.validate_query(prompt) {
            println!("--- Input Query Blocked on Ethical Grounds ---");
            return Some((reason, QueryType::Ambiguous));
        }

        // --- Step 2: Direct Answer Extraction (Common Sense) ---
        if let Some(direct_answer) = self.direct_answer_extractor.extract_direct_answer(prompt, &self.prefrontal_cortex) {
            return Some((direct_answer, QueryType::Factual)); // Classified as Factual, but handled by a shortcut.
        }

        // --- Step 2: Segmentation and Reasoning Strategy ---
        let segments = prompt_segmenter::segment_prompt(prompt);
        let overall_query_type = self.thalamus.analyze_prompt(prompt);

        // --- Step 3: Social Interaction Fast-Path ---
        if overall_query_type == QueryType::Social {
            let intent = social_cortex::SocialCortex::map_prompt_to_intent(prompt);
            let response = self.social_cortex.generate_response(intent);
            return Some((response, QueryType::Social));
        }

        if segments.len() > 1 {
            // --- Stratégie: Agréger les résultats pour une synthèse comparative ---
            let mut all_memories = Vec::new();

            for segment in segments {
                if let Some(mut memories) = self.stimulate_and_reason(&segment) {
                    // On ne garde que la mémoire la plus pertinente pour chaque segment afin d'éviter le bruit
                    // tout en fournissant le contexte nécessaire pour la comparaison.
                    if !memories.is_empty() {
                        memories.truncate(1);
                        all_memories.append(&mut memories);
                    }
                }
            }

            if !all_memories.is_empty() {
                // Envoyer toutes les mémoires collectées au MotorCortex pour une réponse unifiée.
                let response = self.motor_cortex.generate_response(prompt, &Some(all_memories), &self.self_awareness, &self.prefrontal_cortex, &self.conceptual_hierarchy, overall_query_type).unwrap_or_default();
                return Some((response, overall_query_type));
            } else {
                // Fallback si aucune mémoire n'a été trouvée pour aucun segment.
                let response = self.motor_cortex.generate_response(prompt, &None, &self.self_awareness, &self.prefrontal_cortex, &self.conceptual_hierarchy, overall_query_type).unwrap_or_default();
                return Some((response, overall_query_type));
            }

        } else {
            // --- Strategy: DirectReasoning for a single question ---
            if let Some(memories) = self.stimulate_and_reason(prompt) {
                if !memories.is_empty() {
                    // The prefrontal cortex synthesizes the core idea, but the motor cortex has the final word on delivery.
                    let response = self.motor_cortex.generate_response(prompt, &Some(memories), &self.self_awareness, &self.prefrontal_cortex, &self.conceptual_hierarchy, overall_query_type).unwrap_or_default();
                    return Some((response, overall_query_type));
                }
            }
        }

        // --- Default fallback if no reasoning path yielded a result ---
        let response = self.motor_cortex.generate_response(prompt, &None, &self.self_awareness, &self.prefrontal_cortex, &self.conceptual_hierarchy, overall_query_type).unwrap_or_default();
        self.last_reasoning_result = Some(response.clone());
        Some((response, overall_query_type))
    }

    /// Internal reasoning function, separated for clarity.
    fn stimulate_and_reason(&mut self, prompt: &str) -> Option<Vec<HolographicMemory>> {
        // Decompose the prompt into sub-questions for more nuanced processing.
        if let Some(solver) = &self.mcq_solver {
            if let Some(answer_memory) = solver.solve(prompt, &self.hippocampus, &self.holographic_encoder) {
                return Some(vec![answer_memory]);
            }
        }

        let sub_prompts = prompt_segmenter::segment_prompt(prompt);
        let mut combined_results: Vec<HolographicMemory> = Vec::new();

        for sub_prompt in sub_prompts {
            let trimmed_prompt = sub_prompt.trim();
            if trimmed_prompt.is_empty() {
                continue;
            }

            let query_type = self.thalamus.analyze_prompt(trimmed_prompt);
            println!(
                "--- Thalamus classified sub-query '{}' as: {:?} ---",
                trimmed_prompt,
                query_type
            );

            let is_introspective = query_type == QueryType::Introspective;

            // --- Neuro-Modulation: Calcul du seuil de raisonnement dynamique ---
            const BASE_REASONING_THRESHOLD: f32 = 0.95;
            let dynamic_threshold = self.neurochemical_modulator.get_reasoning_distance_threshold(BASE_REASONING_THRESHOLD);
            println!(
                "--- Neuro-Modulation: Reasoning with dynamic threshold: {:.4} (Dopamine: {:.2}) ---",
                dynamic_threshold, self.neurochemical_modulator.state.dopamine
            );

            if let Some(results) = self.reasoning_engine.lock().unwrap().process(
                trimmed_prompt,
                &self.hippocampus,
                &self.conceptual_hierarchy,
                &Arc::clone(&self.holographic_encoder),
                is_introspective,
                dynamic_threshold, // Le seuil dynamique est maintenant utilisé ici
            ) {
                combined_results.extend(results);
            }
        }

        if combined_results.is_empty() {
            None
        } else {
            // --- Neuro-Feedback Loop ---
            // A successful reasoning attempt is a desirable outcome. We reinforce this by a dopamine reward.
            self.neurochemical_modulator.reward_successful_reasoning();
            Some(combined_results)
        }
    }

    /// Teaches the AGI a new hierarchical relationship between two concepts.
    ///
    /// This method is robust: if the concepts do not already exist, they will be
    /// created on-the-fly before the relationship is established.
    ///
    /// # Arguments
    /// * `child_name` - The name of the more specific concept (e.g., "Poodle").
    /// * `parent_name` - The name of the more abstract concept (e.g., "Dog").
    pub fn learn_relationship(&mut self, child_name: &str, parent_name: &str) {
        let encoder = self.holographic_encoder.read().unwrap();

        // Create traces for concepts. `add_concept` will use them only if the concept is new.
        let child_trace = encoder.encode(child_name);
        let parent_trace = encoder.encode(parent_name);

        // Ensure both concepts exist, creating them if necessary.
        let child_id = self.conceptual_hierarchy.add_concept(child_name, child_trace, &[]);
        let parent_id = self.conceptual_hierarchy.add_concept(parent_name, parent_trace, &[]);

        // Drop the read lock before making a mutable call to the hierarchy.
        drop(encoder);

        // Now, establish the hierarchical relationship.
        self.conceptual_hierarchy.add_relationship(child_id, parent_id);

        println!("Successfully linked '{}' as a child of '{}'", child_name, parent_name);
    }

    // --- Phase 1: Biomimetic Wakeup Sequence ---

    pub fn set_wakeup_stages(&mut self, stages: u32) {
        self.wakeup_stages = stages;
        self.current_wakeup_stage = 0;
        println!("Wakeup sequence initiated with {} stages.", stages);
    }

    pub fn advance_wakeup_stage(&mut self) -> bool {
        if self.current_wakeup_stage < self.wakeup_stages {
            self.current_wakeup_stage += 1;
            println!("Entering wakeup stage {}/{}", self.current_wakeup_stage, self.wakeup_stages);
            self.activate_neural_columns();
            self.replay_core_memories();
            self.diffuse_quantum_awareness();
            true
        } else {
            println!("Wakeup sequence complete. AGI is fully operational.");
            false
        }
    }

    fn activate_neural_columns(&mut self) {
        let activation_ratio = self.current_wakeup_stage as f32 / self.wakeup_stages as f32;
        println!("  -> Activating neural columns (ratio: {:.2})...", activation_ratio);

        let num_neurons_to_activate = (self.connectome.neurons.len() as f32 * activation_ratio) as usize;

        // Activate a subset of neurons by setting their potential to the firing threshold.
        // This ensures they will fire on the next `tick`.
        for neuron in self.connectome.neurons.iter_mut().take(num_neurons_to_activate) {
            neuron.potential = neuron.threshold; // Set potential to exactly the threshold
        }

        println!("     - Stimulated {} neurons.", num_neurons_to_activate);
    }

    fn replay_core_memories(&mut self) {
        let replay_intensity = 20.0; // As per Instructions.txt
        println!("  -> Replaying core memories (intensity: {}x)...", replay_intensity);
        self.hippocampus.replay_core_memories(&mut self.quantum_core);
    }

    /// High-level API to load and process a knowledge file.
    pub fn learn_from_file<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        self.knowledge_explorer.load_and_process_file(path)?;
        // Immediately try to assimilate the newly loaded knowledge.
        self.assimilate_knowledge();
        Ok(())
    }


    /// Assimilates new knowledge into the AGI's knowledge base.
    ///
    /// This function processes the concepts loaded by the KnowledgeExplorer, encodes them as holographic memories,
    /// and integrates them into the AGI's knowledge base. It also rebuilds the document frequency map to include the new knowledge.
    pub fn assimilate_knowledge(&mut self) {
        let concepts_to_learn = self.knowledge_explorer.get_discovered_concepts();
        if concepts_to_learn.is_empty() {
            println!("ASSIMILATE: No new concepts to assimilate.");
            return;
        }

        println!("ASSIMILATE: Assimilating {} new concepts...", concepts_to_learn.len());
        for concept_text in concepts_to_learn {
            // The primary `learn` method correctly handles encoding, potentiation, and storing the full HolographicMemory.
            self.learn_and_assimilate(&concept_text, false);
        }

        // After assimilation, it's crucial to rebuild the semantic context.
        println!("ASSIMILATE: Rebuilding document frequency map and Thalamus prototypes...");
        self.holographic_encoder.write().unwrap().build_document_frequency(&self.hippocampus.holographic_memory);
        self.rebuild_thalamus_prototypes();

        // Finally, update the self-awareness module with the new knowledge state.
        self.self_awareness.update_knowledge_summary(&self.hippocampus);
        println!("ASSIMILATE: Knowledge assimilation complete and self-awareness updated.");
    }

    fn diffuse_quantum_awareness(&mut self) {
        let awareness_level = self.current_wakeup_stage as f32 / self.wakeup_stages as f32;
        println!("  -> Diffusing quantum awareness (level: {:.2})...", awareness_level);

        let num_qubits_to_awaken = (self.quantum_core.len() as f32 * awareness_level) as usize;
        let hadamard_gate = HadamardGate;

        // Apply a Hadamard gate to a growing subset of qubits to bring them into superposition gradually.
        for qubit in self.quantum_core.iter_mut().take(num_qubits_to_awaken) {
            // Avoid re-initializing qubits that might already be in superposition.
            if qubit.alpha.re == 1.0 && qubit.beta.re == 0.0 { // Check if it's in the |0> state
                hadamard_gate.apply(qubit);
            }
        }

        println!("     - Awakened {} qubits.", num_qubits_to_awaken);

        // The AGI is considered fully initialized only when the final stage is complete.
        self.quantum_state_initialized = awareness_level >= 1.0;
    }

    pub fn get_awakening_level(&self) -> f32 {
        if self.wakeup_stages == 0 {
            return if self.quantum_state_initialized { 1.0 } else { 0.0 };
        }
        self.current_wakeup_stage as f32 / self.wakeup_stages as f32
    }


    /// Processes an external text input, stimulating neurons and storing the information as a holographic memory.
    pub fn process_external_stimulus(&mut self, text: &str) {
        println!("\n--- Processing External Stimulus: '{}' ---", text);
        self.learn_and_assimilate(text, false);
        println!("--- Stimulus Processed and Learned as Conceptual Memory ---");
    }

    /// Returns a vector of neuron potentials for EEG visualization.
    /// It will return up to `num_points` values.
    pub fn get_eeg_potentials(&self, num_points: usize) -> Vec<f32> {
        self.connectome.neurons.iter()
            .take(num_points)
            .map(|n| n.potential)
            .collect()
    }

    /// Calculates the sum of all electrical potentials in the connectome.
    /// This serves as a raw measure of total brain activity for the EEG visualization.
    pub fn get_total_potential(&self) -> f32 {
        self.connectome.neurons.iter().map(|n| n.potential).sum()
    }


}

impl Default for Core {
    fn default() -> Self {
        // When creating a default Core, we don't load any external knowledge.
        Self::new(None)
    }
}
