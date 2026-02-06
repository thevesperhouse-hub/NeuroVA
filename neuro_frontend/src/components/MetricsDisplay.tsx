"use client";

import { useState, useEffect } from 'react';

interface GpuMetrics {
    name: string;
    usage: number;
}

interface Metrics {
    cpu_usage: number;
    memory_usage_kb: number;
    total_memory_kb: number;
    tps: number;
    concepts_in_memory: number;
        power_draw_w: number;
        // gpus: GpuMetrics[];
}

const MetricsDisplay = () => {
    const [metrics, setMetrics] = useState<Metrics | null>(null);
    const [isConnected, setIsConnected] = useState(false);

    useEffect(() => {
        const ws = new WebSocket('ws://localhost:3000/ws/metrics');

        ws.onopen = () => {
            console.log('Metrics WebSocket connected');
            setIsConnected(true);
        };

        ws.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                setMetrics(data);
            } catch (error) {
                console.error('Failed to parse metrics:', error);
            }
        };

        ws.onclose = () => {
            console.log('Metrics WebSocket disconnected');
            setIsConnected(false);
        };

        ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            setIsConnected(false);
        };

        // Cleanup on component unmount
        return () => {
            ws.close();
        };
    }, []);

    const formatTps = (tps: number) => {
        return tps.toFixed(1);
    };

    const formatMemory = (kb: number) => {
        if (kb < 1024) return `${kb} KB`;
        const mb = kb / 1024;
        if (mb < 1024) return `${mb.toFixed(2)} MB`;
        const gb = mb / 1024;
        return `${gb.toFixed(2)} GB`;
    };

    return (
        <div style={{
            position: 'absolute',
            top: '10px',
            left: '50%',
            transform: 'translateX(-50%)',
            color: '#a0a0a0',
            backgroundColor: 'rgba(0, 0, 0, 0.5)',
            padding: '5px 15px',
            borderRadius: '8px',
            fontFamily: 'monospace',
            fontSize: '14px',
            zIndex: 1000,
            display: 'flex',
            gap: '20px',
            border: isConnected ? '1px solid #2a2a2a' : '1px solid #5a2a2a',
        }}>
            {isConnected && metrics ? (
                <>
                    <span>TPS: {formatTps(metrics.tps)}</span>
                    <span>CPU: {metrics.cpu_usage.toFixed(1)}%</span>
                                        <span>RAM: {formatMemory(metrics.memory_usage_kb)} / {formatMemory(metrics.total_memory_kb)}</span>
                    <span>W: {metrics.power_draw_w.toFixed(2)}</span>
                                        <span>CONCEPTS: {metrics.concepts_in_memory}</span>
                </>
            ) : (
                <span>Connecting to backend...</span>
            )}
        </div>
    );
};

export default MetricsDisplay;
