"use client";

import * as THREE from 'three';
import { useMemo, useRef } from 'react';
import { Canvas, useFrame } from '@react-three/fiber';

function Particles({ count = 5000 }) {
  const pointsRef = useRef<THREE.Points>(null!);
  const bufferRef = useRef<THREE.BufferAttribute>(null!); 

  const { positions, velocities, accelerations } = useMemo(() => {
    const positions = new Float32Array(count * 3);
    const velocities = new Float32Array(count * 3);
    const accelerations = new Float32Array(count * 3);

    for (let i = 0; i < count; i++) {
      positions.set([
        (Math.random() - 0.5) * 10,
        (Math.random() - 0.5) * 10,
        (Math.random() - 0.5) * 10
      ], i * 3);
      velocities.set([0, 0, 0], i * 3);
      accelerations.set([0, 0, 0], i * 3);
    }

    return { positions, velocities, accelerations };
  }, [count]);

  const mouse = new THREE.Vector3(0, 0, 0);
  const vec = new THREE.Vector3();

  useFrame(({ viewport, mouse: { x, y } }) => {
    const aspect = viewport.width / viewport.height;
    mouse.set(x * aspect * 2, y * 2, 0);

    for (let i = 0; i < count; i++) {
      const i3 = i * 3;

      // Update position from buffer
      vec.fromArray(positions, i3);

      // Calculate force direction
      const forceDirection = x < 0 ? 1 : -1; // Attract on left, repel on right
      const dist = vec.distanceTo(mouse);
      const force = (forceDirection / Math.max(0.1, dist * dist)) * 0.015;

      // Apply force
      accelerations[i3] += force * (mouse.x - vec.x);
      accelerations[i3 + 1] += force * (mouse.y - vec.y);
      accelerations[i3 + 2] += force * (mouse.z - vec.z);

      // Update velocity
      velocities[i3] += accelerations[i3];
      velocities[i3 + 1] += accelerations[i3 + 1];
      velocities[i3 + 2] += accelerations[i3 + 2];

      // Update position
      positions[i3] += velocities[i3];
      positions[i3 + 1] += velocities[i3 + 1];
      positions[i3 + 2] += velocities[i3 + 2];

      // Dampen acceleration and velocity
      accelerations[i3] *= 0.98;
      accelerations[i3 + 1] *= 0.98;
      accelerations[i3 + 2] *= 0.98;
      velocities[i3] *= 0.98;
      velocities[i3 + 1] *= 0.98;
      velocities[i3 + 2] *= 0.98;
    }

    bufferRef.current.needsUpdate = true;
  });

  return (
    <points ref={pointsRef}>
      <bufferGeometry>
        <bufferAttribute
          ref={bufferRef}
          attach="attributes-position"
          args={[positions, 3]}
        />
      </bufferGeometry>
      <pointsMaterial
        size={0.015}
        color="#ffffff"
        transparent
        opacity={0.7}
        sizeAttenuation
      />
    </points>
  );
}

export default function ParticleField() {
  return (
    <div className="absolute top-0 left-0 w-full h-full z-0 bg-black">
      <Canvas camera={{ position: [0, 0, 2.5] }}>
        <Particles />
      </Canvas>
    </div>
  );
}
