"use client";

import { useState, useEffect, useRef } from 'react';


interface Message {
  sender: 'user' | 'agi';
  text: string;
  cognitiveMode?: string; // e.g., 'Precision' or 'Creativity'
}

export default function AgiInterface() {
  const [status, setStatus] = useState<'online' | 'offline' | 'loading'>('loading');
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const messagesEndRef = useRef<null | HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }

  useEffect(scrollToBottom, [messages]);

  useEffect(() => {
    const checkStatus = async () => {
      try {
        const response = await fetch('/api/status');
        if (response.ok) {
          const data = await response.json();
          if (data.status === 'ok') {
            setStatus('online');
            setMessages([{ sender: 'agi', text: 'Connexion établie. Je suis prête.' }]);
          } else {
            setStatus('offline');
          }
        } else {
          setStatus('offline');
        }
      } catch (error) {
        setStatus('offline');
        setMessages([{ sender: 'agi', text: 'Erreur de connexion au serveur AGI. Veuillez vérifier qu\'il est bien démarré.' }]);
      }
    };
    checkStatus();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || status !== 'online') return;

    const userMessage: Message = { sender: 'user', text: input };
    setMessages(prev => [...prev, userMessage]);
    setInput('');

    try {
      const response = await fetch('/api/stimulate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: input }),
      });

      if (response.ok) {
                const data = await response.json();
        const agiResponse: Message = { 
          sender: 'agi', 
          text: data.response || 'Je n\'ai pas de réponse pour le moment.',
          cognitiveMode: data.cognitive_mode
        };
        setMessages(prev => [...prev, agiResponse]);
      } else {
        const agiResponse: Message = { sender: 'agi', text: 'Erreur lors de la stimulation de l\'AGI.' };
        setMessages(prev => [...prev, agiResponse]);
      }
    } catch (error) {
      const agiResponse: Message = { sender: 'agi', text: 'Impossible de contacter le serveur pour la stimulation.' };
      setMessages(prev => [...prev, agiResponse]);
    }
  };

  return (
    <div className="relative w-full h-screen overflow-hidden">


      <div className="relative z-10 flex flex-col h-screen text-white font-sans bg-transparent">
        <header className="p-4 flex justify-between items-center bg-black/30 backdrop-blur-md border-b border-white/20 shadow-lg">
          <h1 className="text-xl font-bold">NeuroVA AGI</h1>
          <div className="flex items-center space-x-2">
            <span className={`h-3 w-3 rounded-full ${status === 'online' ? 'bg-green-400' : status === 'offline' ? 'bg-red-500' : 'bg-yellow-400'}`}></span>
            <span className="text-sm capitalize">{status}</span>
          </div>
        </header>

        <main className="flex-1 overflow-y-auto p-4 space-y-4">
          {messages.map((msg, index) => (
            <div key={index} className={`flex ${msg.sender === 'user' ? 'justify-end' : 'justify-start'}`}>
              <div className={`max-w-lg px-4 py-2 rounded-xl shadow-md ${msg.sender === 'user' ? 'bg-blue-500/50' : 'bg-gray-900/50'} backdrop-blur-lg border border-white/10`}>
                                <p className="text-sm whitespace-pre-wrap">{msg.text}</p>
                {msg.sender === 'agi' && msg.cognitiveMode && (
                  <p className="text-xs text-white/50 mt-2 font-mono">Mode: {msg.cognitiveMode}</p>
                )}
              </div>
            </div>
          ))}
          <div ref={messagesEndRef} />
        </main>

        <footer className="p-4 bg-black/30 backdrop-blur-md border-t border-white/20 shadow-lg">
          <form onSubmit={handleSubmit} className="flex space-x-2">
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder={status === 'online' ? 'Posez votre question...' : 'Serveur hors ligne'}
              className="flex-1 p-2 bg-gray-900/50 backdrop-blur-lg border border-white/10 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-400 disabled:opacity-50 text-white"
              disabled={status !== 'online'}
            />
            <button
              type="submit"
              className="px-4 py-2 bg-blue-600/70 backdrop-blur-lg border border-white/10 rounded-md hover:bg-blue-700/80 disabled:bg-gray-500/50 disabled:cursor-not-allowed"
              disabled={status !== 'online' || !input.trim()}
            >
              Envoyer
            </button>
          </form>
        </footer>
      </div>
    </div>
  );
}
