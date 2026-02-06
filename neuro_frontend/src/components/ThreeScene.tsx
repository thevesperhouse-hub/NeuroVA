"use client";
import React, { useRef, useEffect } from 'react';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader';
import { GLTF } from 'three/examples/jsm/loaders/GLTFLoader';

const ThreeScene: React.FC = () => {
    const mountRef = useRef<HTMLDivElement>(null);
    const particleSystemRef = useRef<THREE.Points | null>(null);

    useEffect(() => {
        if (!mountRef.current) return;

        const currentMount = mountRef.current;

        // Scene
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x0a0a0a); // Dark background for the ethereal vibe

        // Camera
        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        camera.position.z = 20; // Bring camera closer for a more intimate view

        // Renderer
        const renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(window.innerWidth, window.innerHeight);
        currentMount.appendChild(renderer.domElement);

        // Controls
        const controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;

        // Lighting
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
        scene.add(ambientLight);

        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
        directionalLight.position.set(5, 10, 7.5);
        scene.add(directionalLight);





        // Load 3D model
        const loader = new GLTFLoader();
        loader.load(
            '/models/avatar.glb', // Path relative to the public directory
            (gltf: any) => {
                const model = gltf.scene;

                const allVertices: number[] = [];
                const allColors: number[] = [];
                const colorPalette = [new THREE.Color(0x00aaff), new THREE.Color(0xff00ff), new THREE.Color(0x00ffdd)];

                model.updateMatrixWorld(true); // Ensure world matrices are up-to-date

                model.traverse((child: any) => {
                    if (child.isMesh) {
                        const positions = child.geometry.attributes.position.array;
                        for (let i = 0; i < positions.length; i += 3) {
                            const vertex = new THREE.Vector3(positions[i], positions[i+1], positions[i+2]);
                            vertex.applyMatrix4(child.matrixWorld);
                            allVertices.push(vertex.x, vertex.y, vertex.z);

                            // Assign color based on normalized Y position
                            const normalizedY = (vertex.y - model.position.y) / 10; // Adjust divisor for gradient spread
                            const color = colorPalette[0].clone().lerp(colorPalette[1], normalizedY);
                            const finalColor = color.clone().lerp(colorPalette[2], normalizedY * 0.5);
                            allColors.push(finalColor.r, finalColor.g, finalColor.b);
                        }
                    }
                });

                if (allVertices.length > 0) {
                    const combinedGeometry = new THREE.BufferGeometry();
                    combinedGeometry.setAttribute('position', new THREE.Float32BufferAttribute(allVertices, 3));
                    combinedGeometry.setAttribute('color', new THREE.Float32BufferAttribute(allColors, 3));

                    // Center the combined geometry
                    combinedGeometry.center();

                    // Helper function to create a procedural particle texture
                    const createParticleTexture = () => {
                        const canvas = document.createElement('canvas');
                        canvas.width = 128;
                        canvas.height = 128;
                        const context = canvas.getContext('2d')!;
                        const gradient = context.createRadialGradient(canvas.width / 2, canvas.height / 2, 0, canvas.width / 2, canvas.height / 2, canvas.width / 2);
                        gradient.addColorStop(0, 'rgba(255,255,255,1)');
                        gradient.addColorStop(0.2, 'rgba(255,255,255,0.8)');
                        gradient.addColorStop(1, 'rgba(255,255,255,0)');
                        context.fillStyle = gradient;
                        context.fillRect(0, 0, canvas.width, canvas.height);
                        return new THREE.CanvasTexture(canvas);
                    };

                    const particlesMaterial = new THREE.PointsMaterial({
                        vertexColors: true, // Enable vertex colors
                        size: 0.07, // Refined particle size for a more delicate look
                        map: createParticleTexture(),
                        blending: THREE.AdditiveBlending,
                        transparent: true,
                        depthWrite: false, // Important for correct blending
                        sizeAttenuation: true
                    });

                    const particleSystem = new THREE.Points(combinedGeometry, particlesMaterial);

                    // Adjust scale and position for a better fit
                    particleSystem.scale.set(15, 15, 15);
                    particleSystem.position.y = -8;

                    scene.add(particleSystem);
                    // Store a reference and the original positions for animation
                    particleSystem.userData.originalPositions = combinedGeometry.attributes.position.clone();
                    particleSystemRef.current = particleSystem;
                    console.log(`Particle avatar created with ${allVertices.length / 3} vertices.`);
                } else {
                    console.error('No mesh found in the model to create particles from.');
                    // Fallback to showing the original model if no particles could be created
                    scene.add(model);
                }
            },
            undefined, // onProgress callback (optional)
            (error: any) => {
                console.error('An error happened while loading the model:', error);
            }
        );

        // Handle window resize
        const handleResize = () => {
            camera.aspect = window.innerWidth / window.innerHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(window.innerWidth, window.innerHeight);
        };
        window.addEventListener('resize', handleResize);

        // Animation loop
        const animate = () => {
            requestAnimationFrame(animate);

            // Add breathing/shimmering animation to particles
            if (particleSystemRef.current) {
                const time = Date.now() * 0.0005;
                const positions = particleSystemRef.current.geometry.attributes.position as THREE.BufferAttribute;
                const originalPositions = particleSystemRef.current.userData.originalPositions as THREE.BufferAttribute;
                const count = positions.count;

                for (let i = 0; i < count; i++) {
                    const originalX = originalPositions.getX(i);
                    const originalY = originalPositions.getY(i);
                    const originalZ = originalPositions.getZ(i);

                    // More organic, multi-layered distortion
                    const f = 0.15; // frequency
                    const a = 0.2; // amplitude
                    const xOffset = Math.sin(originalY * f + time * 0.5) * Math.cos(originalZ * f + time * 0.5) * a;
                    const yOffset = Math.cos(originalX * f + time * 0.5) * Math.sin(originalZ * f + time * 0.5) * a;
                    const zOffset = Math.sin(originalX * f + time * 0.5) * Math.cos(originalY * f + time * 0.5) * a;

                    positions.setX(i, originalX + xOffset);
                    positions.setY(i, originalY + yOffset);
                    positions.setZ(i, originalZ + zOffset);
                }
                positions.needsUpdate = true;
            }

            controls.update();
            renderer.render(scene, camera);
        };
        animate();

        // Cleanup
        return () => {
            window.removeEventListener('resize', handleResize);
            if (currentMount) {
                currentMount.removeChild(renderer.domElement);
            }
        };
    }, []);

    return <div ref={mountRef} style={{ width: '100%', height: '100%', position: 'absolute', top: 0, left: 0, zIndex: -1 }} />;
};

export default ThreeScene;

