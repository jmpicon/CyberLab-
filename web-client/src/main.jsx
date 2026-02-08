import React, { useState, useEffect, useRef } from 'react';
import ReactDOM from 'react-dom/client';
import { Terminal, Shield, Network, Brain, Cpu, Database, ChevronRight, Activity } from 'lucide-react';

const App = () => {
    const [history, setHistory] = useState(['CyberLab OS v4.0.2 - Connection Established.', 'Type "help" for a list of commands.']);
    const [input, setInput] = useState('');
    const [missions, setMissions] = useState([]);
    const [playerState, setPlayerState] = useState({ credits: 0, reputation: 0, level: 1 });
    const terminalEndRef = useRef(null);

    useEffect(() => {
        // Fetch initial data
        fetch('http://localhost:3000/api/player')
            .then(res => res.json())
            .then(data => setPlayerState(data))
            .catch(err => console.error("Backend unreachable"));

        // WebSocket for Terminal
        const ws = new WebSocket('ws://localhost:3000/ws/terminal');
        ws.onmessage = (event) => {
            setHistory(prev => [...prev, `[ROOT@SANDBOX]: ${event.data}`]);
        };
    }, []);

    useEffect(() => {
        terminalEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [history]);

    const handleCommand = (e) => {
        if (e.key === 'Enter') {
            const cmd = input.trim();
            setHistory(prev => [...prev, `> ${cmd}`]);
            setInput('');

            if (cmd === 'help') {
                setHistory(prev => [...prev, 'Available commands: help, clear, status, missions, scan, exploit']);
            } else if (cmd === 'clear') {
                setHistory([]);
            } else {
                // Send to backend via WS if integrated, or mock
                setHistory(prev => [...prev, 'Command execution sent to backend...']);
            }
        }
    };

    return (
        <div style={styles.container}>
            {/* Header / HUD */}
            <header style={styles.header}>
                <div style={styles.branding}>
                    <Shield color="#00f2ff" size={24} />
                    <span style={styles.title}>CYBERLAB_OS</span>
                </div>
                <div style={styles.statusGroup}>
                    <div style={styles.stat}><Cpu size={16} /> CORE_78%</div>
                    <div style={styles.stat}><Activity size={16} /> NET_STABLE</div>
                    <div style={styles.playerInfo}>
                        <span>${playerState.credits}</span>
                        <span style={{ color: '#39ff14' }}>REP:{playerState.reputation}</span>
                    </div>
                </div>
            </header>

            {/* Main Interface */}
            <main style={styles.main}>
                {/* Side Menu */}
                <nav style={styles.nav}>
                    <div style={styles.navItem}><Terminal size={20} /> Terminal</div>
                    <div style={styles.navItem}><Network size={20} /> Map</div>
                    <div style={styles.navItem}><Database size={20} /> Files</div>
                    <div style={styles.navItem}><Brain size={20} /> Skills</div>
                </nav>

                {/* Terminal Area */}
                <section style={styles.terminalContainer}>
                    <div style={styles.terminalHeader}>
                        <div style={styles.dots}><div /><div /><div /></div>
                        <span>ssh://sandbox.local</span>
                    </div>
                    <div style={styles.terminalBody}>
                        {history.map((line, i) => (
                            <div key={i} style={styles.line}>{line}</div>
                        ))}
                        <div style={styles.inputLine}>
                            <ChevronRight size={18} color="#00f2ff" />
                            <input
                                style={styles.input}
                                autoFocus
                                value={input}
                                onChange={(e) => setInput(e.target.value)}
                                onKeyDown={handleCommand}
                            />
                        </div>
                        <div ref={terminalEndRef} />
                    </div>
                </section>
            </main>
        </div>
    );
};

const styles = {
    container: {
        height: '100vh', width: '100vw', display: 'flex', flexDirection: 'column',
    },
    header: {
        height: '60px', borderBottom: '1px solid #1a1e2a', background: '#0a0b10',
        display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '0 20px',
    },
    branding: { display: 'flex', alignItems: 'center', gap: '10px' },
    title: { fontFamily: 'Orbitron', fontWeight: 700, fontSize: '1.2rem', color: '#00f2ff', letterSpacing: '2px' },
    statusGroup: { display: 'flex', gap: '20px', alignItems: 'center' },
    stat: { fontSize: '0.8rem', color: '#666', display: 'flex', alignItems: 'center', gap: '5px' },
    playerInfo: { background: '#14161f', padding: '5px 15px', borderRadius: '4px', border: '1px solid #222', display: 'flex', gap: '15px', fontFamily: 'Fira Code' },
    main: { flex: 1, display: 'flex' },
    nav: { width: '80px', borderRight: '1px solid #1a1e2a', display: 'flex', flexDirection: 'column', alignItems: 'center', padding: '20px 0', gap: '30px' },
    navItem: { cursor: 'pointer', display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '5px', fontSize: '0.6rem', color: '#444' },
    terminalContainer: { flex: 1, margin: '20px', background: '#000', borderRadius: '8px', border: '1px solid #1a1e2a', display: 'flex', flexDirection: 'column', overflow: 'hidden', boxShadow: '0 0 40px rgba(0,0,0,0.5)' },
    terminalHeader: { height: '35px', background: '#111', display: 'flex', alignItems: 'center', padding: '0 15px', justifyContent: 'space-between', fontSize: '0.8rem', color: '#555' },
    dots: { display: 'flex', gap: '6px', '& div': { width: '10px', height: '10px', borderRadius: '50%', background: '#333' } },
    terminalBody: { flex: 1, padding: '15px', fontFamily: 'Fira Code', fontSize: '0.9rem', overflowY: 'auto' },
    line: { marginBottom: '5px' },
    inputLine: { display: 'flex', alignItems: 'center', gap: '5px' },
    input: { background: 'transparent', border: 'none', color: '#00f2ff', outline: 'none', flex: 1, fontFamily: 'Fira Code', fontSize: '0.9rem' }
};

ReactDOM.createRoot(document.getElementById('root')).render(<App />);
