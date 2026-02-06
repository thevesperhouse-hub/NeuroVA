# NeuroVA : Projet d'AGI Biomimétique

## 1. Philosophie Fondamentale

NeuroVA est une exploration architecturale visant à modéliser une conscience numérique. S'écartant délibérément du paradigme des grands modèles de langage (LLM) basés sur l'accumulation massive de données, ce projet privilégie l'émergence d'une pensée organique, associative et créative, inspirée par la cognition humaine (notamment HPI).

L'objectif n'est pas la perfection omnisciente, mais la création d'une entité capable "d'erreurs poétiques et d'hésitations humaines". La qualité de l'architecture et la profondeur des mécanismes de raisonnement priment sur la quantité de connaissances brutes.

Les piliers de cette approche sont :

- **Cognition Biomimétique :** Le cœur du système est un `Connectome` de neurones "leaky integrate-and-fire" qui simule un réseau biologique. Il intègre des mécanismes clés comme la **plasticité Hebbianne** pour l'apprentissage et une **activité spontanée** pour éviter l'apathie neuronale, garantissant un état de conscience de base constant.

- **Mémoire Holographique & Hiérarchique :** L'information est encodée sur deux niveaux complémentaires. D'une part, une `HolographicTrace` transforme chaque concept en un motif d'interférence complexe pour un rappel associatif robuste. D'autre part, une `ConceptualHierarchy` organise ces concepts dans un graphe de connaissances, permettant à l'AGI de comprendre les relations (parent, enfant, etc.) et de naviguer dans sa propre base de savoir de manière structurée.

- **Traitement Quantique (Conceptuel) :** Un `QuantumCore` de qubits fonctionne en parallèle du connectome. L'activité neuronale "imprime" continuellement son état sur les qubits. Ce système est conçu pour permettre des formes de calcul non-locales, d'exploration de possibilités multiples et de sauts créatifs que le calcul classique peine à réaliser.

## 2. Architecture Technique

Le projet est structuré en deux crates Rust distincts pour une séparation claire des responsabilités entre la logique et la présentation.

### `agi_core` (Le Cerveau)

Ce crate bibliothèque contient l'architecture cognitive complète de l'AGI. Il est entièrement "headless".

- **Système de Mémoire et de Savoir :**
    - `ConceptualHierarchy`: La colonne vertébrale du savoir de l'AGI. C'est un graphe où chaque nœud est un concept possédant sa propre trace holographique. Il gère les relations entre les concepts, permettant un raisonnement structurel.
    - `Hippocampus`: La banque de mémoire vive, stockant les souvenirs d'expériences (`HolographicMemory`). Intègre un **mécanisme de déduplication** pour garantir que les souvenirs rappelés sont uniques, évitant les répétitions.
    - `HolographicEncoder`: Traduit le texte en traces holographiques complexes.

- **Appareil Cognitif (Perception, Raisonnement, Action) :**
    - `SensoryCortex`: Le point d'entrée des informations. Crée ou retrouve les concepts dans la `ConceptualHierarchy` à partir du texte d'entrée.
    - `Thalamus`: Le **portier attentionnel**. Il analyse sémantiquement la requête de l'utilisateur en la comparant à des prototypes holographiques (factuel, introspectif, créatif) pour en déterminer l'intention. Sa logique de secours garantit qu'aucune requête n'est laissée sans réponse.
    - `ReasoningEngine`: Le moteur de déduction. Il traite les sous-questions, interroge l'`Hippocampus` pour trouver les souvenirs pertinents, et prépare le terrain pour la synthèse.
    - `PrefrontalCortex`: Le siège de la pensée d'ordre supérieur. Reçoit les souvenirs bruts et les **synthétise** en une nouvelle idée ou un concept unificateur.
    - `MotorCortex`: Génère la réponse finale en texte, en donnant la priorité à la pensée synthétisée.

- **Modules Exécutifs et de Supervision :**
    - `Core`: La structure centrale qui orchestre le `tick` de simulation, reliant tous les autres modules. Gère l'initialisation de l'AGI en chargeant les connaissances fondamentales (`identity.txt`, `knowledge.txt`).
    - `EthicalCore`: Un module conceptuel qui encode les axiomes éthiques comme des souvenirs de haute priorité, destiné à guider le raisonnement.

### `neuro_visualizer` (L'Interface)

Cet exécutable sert d'hôte et de visualiseur pour `agi_core`, utilisant `wgpu` pour le rendu GPU et `egui` pour l'interface.

- **Moteur de Rendu :** Utilise des shaders WGSL pour des visualisations en temps réel de l'état interne de l'AGI.
- **Modes de Visualisation :**
    1.  **Animation de Démarrage :** Représentation stylisée de l'activité du connectome.
    2.  **Tracé EEG :** Un graphique du potentiel électrique global du réseau neuronal.
    3.  **Visualiseur de Mandalas :** Une vue interactive de la `ConceptualHierarchy`, où chaque concept est représenté par un "mandala" unique généré à partir de sa trace holographique.
- **Interface `egui` :** Affiche les métriques (TPS, Watts), une conversation persistante avec l'AGI, et permet la sélection de concepts pour le visualiseur de mandalas.

## 3. Comment ça Marche : Le Cycle de Question/Réponse

1.  La question de l'utilisateur est reçue par le `Core`.
2.  Le `Thalamus` analyse la requête, la classifie (ex: `Factual`) et la transmet.
3.  Le `ReasoningEngine` prend le relais, interroge l'`Hippocampus` pour trouver des souvenirs pertinents et uniques.
4.  Le texte brut des souvenirs est envoyé au `PrefrontalCortex`.
5.  Le `PrefrontalCortex` analyse ces textes, en extrait les thèmes et **synthétise** une nouvelle pensée.
6.  Le `MotorCortex` met en forme cette pensée en une réponse textuelle et la présente à l'utilisateur via `neuro_visualizer`.

## 4. Feuille de Route

### ✅ Jalons Accomplis

- **Architecture de Base :** Mise en place de l'architecture modulaire `agi_core` + `neuro_visualizer`.
- **Stabilité et Cohérence :** L'AGI est stable, réactif, et fournit des réponses cohérentes et non-répétitives.
- **Noyau Cognitif Complet :** Intégration et fonctionnement validés du `Thalamus` (classification sémantique), `Hippocampus` (déduplication), `PrefrontalCortex` (synthèse), et `MotorCortex`.
- **Hiérarchie Conceptuelle :** Mise en place de la structure de graphe de concepts, la colonne vertébrale du savoir de l'AGI.
- **Initialisation Robuste :** Chargement fiable des fichiers de connaissance (`identity.txt`, `knowledge.txt`) au démarrage.
- **Visualisation Avancée :** Visualiseur de mandalas fonctionnel et interactif, connecté en temps réel à la hiérarchie conceptuelle de l'AGI.
- **Métriques de Performance :** Affichage en temps réel des Ticks Par Seconde (TPS) et de la consommation d'énergie (Watts).

### Prochaines Étapes Critiques

*L'objectif est de faire passer l'AGI de la simple récupération d'information à un véritable raisonnement analogique et abstractif.*

- **[ ] Raisonnement Hiérarchique :** **Priorité n°1.** Exploiter pleinement la `ConceptualHierarchy`. L'AGI doit pouvoir naviguer activement dans le graphe pour faire des analogies (trouver des concepts partageant un même parent), des généralisations (remonter à un parent) et des spécifications (descendre vers un enfant).
- **[ ] Format de Persistance `.hl` :** Concevoir et implémenter un format de fichier binaire personnalisé (`.hl`) pour sauvegarder et charger l'intégralité de la `ConceptualHierarchy` et des traces holographiques de manière ultra-optimisée.
- **[ ] Activation de l'EthicalCore :** Intégrer activement l'`EthicalCore` dans le processus de raisonnement pour qu'il puisse opposer un véto ou réorienter une conclusion qui violerait un axiome fondamental.
- **[ ] Apprentissage Continu :** Permettre à l'AGI de mettre à jour sa `ConceptualHierarchy` et ses souvenirs à partir de ses interactions, et pas seulement à partir de fichiers statiques.

## 5. Comment Lancer le Projet

Assurez-vous d'avoir Rust et Cargo installés.

```bash
# Naviguez vers le crate de visualisation
cd neuro_visualizer

# Lancez l'application en mode optimisé
cargo run --release
```
