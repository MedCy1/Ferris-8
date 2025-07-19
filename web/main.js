// 🦀 Ferris-8 - Interface JavaScript complète et sécurisée

import init, { Emulator, greet } from './pkg/ferris8.js';

class Ferris8App {
    constructor() {
        this.emulator = null;
        this.animationId = null;
        this.running = false;
        this.fps = 0;
        this.lastFpsUpdate = 0;
        this.frameCount = 0;
        this.cyclesPerSecond = 500;
        this.lastCycleCount = 0;
        this.actualSpeed = 0;

        this.canvas = document.getElementById('display');
        this.ctx = this.canvas.getContext('2d');
        this.loadingEl = document.getElementById('loading');

        this.setupCanvas();

        this.bindEvents();

        this.pressedKeys = new Set();

        this.predefinedROMs = {
            'simple': this.createSimpleROM,
            'blink': this.createBlinkROM,
            'test': this.createTestROM,
            'demo': this.createDemoROM
        };

        this.errorCount = 0;
        this.lastError = null;

        console.log('🦀 Ferris-8 App créée avec gestion d\'erreurs renforcée');
    }

    async init() {
        try {
            console.log('📦 Chargement du module WASM...');

            // Initialiser le module WebAssembly
            await init();

            const greeting = greet('Developer');
            console.log(greeting);

            // Créer l'émulateur
            this.emulator = new Emulator();
            console.log('✅ Émulateur créé');

            // Vérifier que l'émulateur fonctionne
            const debugInfo = this.emulator.get_debug_info();
            console.log('🔍 État initial:', debugInfo);

            this.hideLoading();

            this.updateDebugInfo();
            this.updateStatus('🟡 Prêt');

            console.log('🚀 Ferris-8 prêt à l\'emploi !');

        } catch (error) {
            console.error('❌ Erreur lors de l\'initialisation:', error);
            this.handleError('Erreur de chargement', error);
        }
    }

    setupCanvas() {
        // Configuration du rendu pixelisé
        this.ctx.imageSmoothingEnabled = false;
        this.ctx.webkitImageSmoothingEnabled = false;
        this.ctx.mozImageSmoothingEnabled = false;
        this.ctx.msImageSmoothingEnabled = false;

        // Fond noir initial
        this.clearCanvas();

        // Afficher un message d'attente
        this.ctx.fillStyle = '#333333';
        this.ctx.font = '16px monospace';
        this.ctx.textAlign = 'center';
        this.ctx.fillText('Ferris-8 Ready', 320, 160);
    }

    bindEvents() {
        // Boutons de contrôle
        document.getElementById('btn-start').addEventListener('click', () => this.start());
        document.getElementById('btn-stop').addEventListener('click', () => this.stop());
        document.getElementById('btn-reset').addEventListener('click', () => this.reset());

        // Chargement de ROM
        document.getElementById('rom-input').addEventListener('change', (e) => this.loadROM(e));
        
        // Boutons ROMs de test
        document.getElementById('btn-rom-simple').addEventListener('click', () => this.loadTestROM('simple'));
        document.getElementById('btn-rom-blink').addEventListener('click', () => this.loadTestROM('blink'));
        document.getElementById('btn-rom-test').addEventListener('click', () => this.loadTestROM('test'));
        document.getElementById('btn-rom-demo').addEventListener('click', () => this.loadTestROM('demo'));

        // Paramètres
        document.getElementById('speed-slider').addEventListener('input', (e) => {
            this.cyclesPerSecond = parseInt(e.target.value);
            document.getElementById('speed-value').textContent = this.cyclesPerSecond;
        });

        document.getElementById('sound-enabled').addEventListener('change', (e) => {
            console.log('🔊 Son', e.target.checked ? 'activé' : 'désactivé');
            // TODO: Implémenter contrôle du son
        });

        // Clavier
        document.addEventListener('keydown', (e) => this.handleKeyDown(e));
        document.addEventListener('keyup', (e) => this.handleKeyUp(e));

        // Empêcher les actions par défaut sur certaines touches
        document.addEventListener('keydown', (e) => {
            if (['Space', 'ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.code)) {
                e.preventDefault();
            }
        });

        // Clavier virtuel Chip-8
        document.querySelectorAll('.key').forEach(key => {
            key.addEventListener('mousedown', (e) => {
                e.preventDefault();
                this.handleVirtualKey(key, true);
            });
            key.addEventListener('mouseup', () => this.handleVirtualKey(key, false));
            key.addEventListener('mouseleave', () => this.handleVirtualKey(key, false));
            key.addEventListener('touchstart', (e) => {
                e.preventDefault();
                this.handleVirtualKey(key, true);
            });
            key.addEventListener('touchend', (e) => {
                e.preventDefault();
                this.handleVirtualKey(key, false);
            });
        });

        // Debug
        document.getElementById('btn-memory-dump').addEventListener('click', () => this.memoryDump());

        // Gestion des erreurs globales
        window.addEventListener('error', (e) => {
            this.handleError('Erreur JavaScript', e.error);
        });

        window.addEventListener('unhandledrejection', (e) => {
            this.handleError('Promise rejetée', e.reason);
        });
    }

    // ========== CONTRÔLES DE L'ÉMULATEUR ==========

    start() {
        if (!this.emulator) {
            this.showError('Émulateur pas encore initialisé');
            return;
        }

        if (this.running) {
            console.log('⚠️ Émulateur déjà en cours');
            return;
        }

        try {
            console.log('▶️ Démarrage de l\'émulateur');
            this.emulator.start();
            this.running = true;
            this.errorCount = 0;
            this.runLoop();
            this.updateStatus('🟢 En cours');

            // Mettre à jour les boutons
            document.getElementById('btn-start').disabled = true;
            document.getElementById('btn-stop').disabled = false;

        } catch (error) {
            this.handleError('Erreur de démarrage', error);
        }
    }

    stop() {
        if (!this.running) return;

        try {
            console.log('⏹️ Arrêt de l\'émulateur');
            this.running = false;

            if (this.animationId) {
                cancelAnimationFrame(this.animationId);
                this.animationId = null;
            }

            if (this.emulator) {
                this.emulator.stop();
            }

            this.updateStatus('🔴 Arrêté');

            // Mettre à jour les boutons
            document.getElementById('btn-start').disabled = false;
            document.getElementById('btn-stop').disabled = true;

        } catch (error) {
            this.handleError('Erreur d\'arrêt', error);
        }
    }

    reset() {
        try {
            console.log('🔄 Reset de l\'émulateur');
            const wasRunning = this.running;

            this.stop();

            if (this.emulator) {
                this.emulator.reset();
            }

            this.clearCanvas();
            this.updateDebugInfo();
            this.updateStatus('🔄 Reset');
            this.errorCount = 0;
            this.lastError = null;

            // Redémarrer si c'était en cours
            if (wasRunning) {
                setTimeout(() => this.start(), 200);
            }

        } catch (error) {
            this.handleError('Erreur de reset', error);
        }
    }

    // ========== BOUCLE PRINCIPALE ==========

    runLoop() {
        if (!this.running || !this.emulator) return;

        try {
            const now = performance.now();

            // Calculer le nombre de cycles à exécuter (limité pour éviter les blocages)
            const targetCycles = Math.floor(this.cyclesPerSecond / 60);
            const maxCycles = Math.min(targetCycles, 50);

            // Exécuter les cycles avec protection
            for (let i = 0; i < maxCycles; i++) {
                if (!this.running) break;

                try {
                    this.emulator.cycle();
                } catch (error) {
                    console.error('❌ Erreur dans cycle:', error);
                    this.errorCount++;

                    if (this.errorCount > 5) {
                        this.handleError('Trop d\'erreurs de cycle', error);
                        return;
                    }
                    break;
                }
            }

            // Mise à jour de l'affichage
            this.updateDisplay();
            this.updateDebugInfo();

            // Calcul des performances
            this.frameCount++;
            if (now - this.lastFpsUpdate >= 1000) {
                this.fps = this.frameCount;
                this.frameCount = 0;
                this.lastFpsUpdate = now;

                // Calculer la vitesse réelle
                const debugInfo = this.emulator.get_debug_info();
                const currentCycles = this.extractCycleCount(debugInfo);
                this.actualSpeed = currentCycles - this.lastCycleCount;
                this.lastCycleCount = currentCycles;

                // Mise à jour affichage
                document.getElementById('fps').textContent = `FPS: ${this.fps} | Speed: ${this.actualSpeed}Hz`;
            }

            // Continuer la boucle
            this.animationId = requestAnimationFrame(() => this.runLoop());

        } catch (error) {
            this.handleError('Erreur dans la boucle principale', error);
        }
    }

    extractCycleCount(debugInfo) {
        const match = debugInfo.match(/Cycles: (\d+)/);
        return match ? parseInt(match[1]) : 0;
    }

    // ========== AFFICHAGE ==========

    updateDisplay() {
        if (!this.emulator) return;

        try {
            // Récupérer le buffer d'affichage depuis Rust
            const buffer = this.emulator.get_display_buffer();

            if (!buffer || buffer.length !== 2048) {
                console.warn('⚠️ Buffer d\'affichage invalide:', buffer?.length);
                return;
            }

            // Créer ImageData pour les vrais pixels (64x32)
            const imageData = this.ctx.createImageData(64, 32);

            // Convertir le buffer monochrome en RGBA
            for (let i = 0; i < buffer.length; i++) {
                const pixelValue = buffer[i];
                const baseIndex = i * 4;

                // Blanc si pixel allumé, noir sinon
                imageData.data[baseIndex] = pixelValue; // Rouge
                imageData.data[baseIndex + 1] = pixelValue; // Vert  
                imageData.data[baseIndex + 2] = pixelValue; // Bleu
                imageData.data[baseIndex + 3] = 255; // Alpha (toujours opaque)
            }

            // Créer un canvas temporaire pour le rendu
            const tempCanvas = document.createElement('canvas');
            tempCanvas.width = 64;
            tempCanvas.height = 32;
            const tempCtx = tempCanvas.getContext('2d');
            tempCtx.putImageData(imageData, 0, 0);

            // Effacer le canvas principal
            this.clearCanvas();

            // Redimensionner de 64x32 à 640x320 (x10) avec rendu net
            this.ctx.imageSmoothingEnabled = false;
            this.ctx.drawImage(tempCanvas, 0, 0, 64, 32, 0, 0, 640, 320);

        } catch (error) {
            console.error('❌ Erreur d\'affichage:', error);
            this.errorCount++;
        }
    }

    clearCanvas() {
        this.ctx.fillStyle = '#000000';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    }

    // ========== GESTION DU CLAVIER ==========

    handleKeyDown(event) {
        const chip8Key = this.mapKeyToChip8(event.code);
        if (chip8Key !== null && !this.pressedKeys.has(chip8Key)) {
            this.pressedKeys.add(chip8Key);
            if (this.emulator) {
                this.emulator.key_down(chip8Key);
            }
            this.updateVirtualKeyboard(chip8Key, true);

            // Feedback visuel
            console.log(`⌨️ Touche ${chip8Key.toString(16).toUpperCase()} pressée`);
        }
    }

    handleKeyUp(event) {
        const chip8Key = this.mapKeyToChip8(event.code);
        if (chip8Key !== null && this.pressedKeys.has(chip8Key)) {
            this.pressedKeys.delete(chip8Key);
            if (this.emulator) {
                this.emulator.key_up(chip8Key);
            }
            this.updateVirtualKeyboard(chip8Key, false);
        }
    }

    handleVirtualKey(keyElement, pressed) {
        const keyValue = keyElement.dataset.key;
        const chip8Key = this.hexToChip8(keyValue);

        if (pressed) {
            keyElement.classList.add('pressed');
            if (!this.pressedKeys.has(chip8Key)) {
                this.pressedKeys.add(chip8Key);
                if (this.emulator) {
                    this.emulator.key_down(chip8Key);
                }
            }
        } else {
            keyElement.classList.remove('pressed');
            this.pressedKeys.delete(chip8Key);
            if (this.emulator) {
                this.emulator.key_up(chip8Key);
            }
        }
    }

    mapKeyToChip8(keyCode) {
        const mapping = {
            'Digit1': 0x1, 'Digit2': 0x2, 'Digit3': 0x3, 'Digit4': 0xC,
            'KeyQ': 0x4, 'KeyW': 0x5, 'KeyE': 0x6, 'KeyR': 0xD,
            'KeyA': 0x7, 'KeyS': 0x8, 'KeyD': 0x9, 'KeyF': 0xE,
            'KeyZ': 0xA, 'KeyX': 0x0, 'KeyC': 0xB, 'KeyV': 0xF
        };
        return mapping[keyCode] || null;
    }

    hexToChip8(hex) {
        return parseInt(hex, 16);
    }

    updateVirtualKeyboard(chip8Key, pressed) {
        const keyElement = document.querySelector(`[data-key="${chip8Key.toString(16).toUpperCase()}"]`);
        if (keyElement) {
            if (pressed) {
                keyElement.classList.add('pressed');
            } else {
                keyElement.classList.remove('pressed');
            }
        }
    }

    // ========== CHARGEMENT DE ROM ==========

    async loadROM(event) {
        const file = event.target.files[0];
        if (!file) return;

        try {
            console.log(`📁 Chargement de ${file.name} (${file.size} bytes)...`);

            // Vérifications de base
            if (file.size > 3584) {
                throw new Error(`ROM trop grosse: ${file.size} bytes > 3584 bytes max`);
            }

            if (file.size === 0) {
                throw new Error('ROM vide');
            }

            // Reset AVANT de charger
            this.reset();

            const arrayBuffer = await file.arrayBuffer();
            const romData = new Uint8Array(arrayBuffer);

            // Charger la ROM
            this.emulator.load_rom(romData);
            console.log(`✅ ROM ${file.name} chargée avec succès`);

            // Afficher info sur la ROM
            this.displayROMInfo(file.name, romData);
            this.updateStatus(`🎮 ROM: ${file.name}`);

            // PAS de reset après !

        } catch (error) {
            console.error('❌ Erreur de chargement ROM:', error);
            this.handleError('Erreur de chargement ROM', error);
        }
    }

    loadTestROM(romType) {
        try {
            console.log(`🎮 Chargement ROM: ${romType}`);

            // Reset AVANT de charger
            this.reset();

            let testROM;
            let romName;

            switch(romType) {
                case 'simple':
                    testROM = this.createSimpleROM();
                    romName = 'Pixel fixe';
                    break;
                case 'blink':
                    testROM = this.createBlinkROM();
                    romName = 'Pixel clignotant';
                    break;
                case 'test':
                    testROM = this.createTestROM();
                    romName = 'Carré mobile';
                    break;
                case 'demo':
                    testROM = this.createDemoROM();
                    romName = 'Affichage BE';
                    break;
                default:
                    testROM = this.createSimpleROM();
                    romName = 'Pixel fixe (défaut)';
            }

            this.emulator.load_rom(testROM);
            console.log(`✅ ROM ${romName} chargée: ${testROM.length} bytes`);

            this.displayROMInfo(romName, testROM);
            this.updateStatus(`🎮 ${romName}`);

        } catch (error) {
            this.handleError('Erreur de chargement ROM test', error);
        }
    }

    displayROMInfo(name, romData) {
        const info = `📋 ROM: ${name} | Taille: ${romData.length} bytes | Premières instructions: `;
        let hexDump = '';

        for (let i = 0; i < Math.min(romData.length, 8); i += 2) {
            if (i + 1 < romData.length) {
                const instr = (romData[i] << 8) | romData[i + 1];
                hexDump += `0x${instr.toString(16).toUpperCase().padStart(4, '0')} `;
            }
        }

        console.log(info + hexDump);
    }

    // ========== ROMS DE TEST CORRIGÉES ==========

    createSimpleROM() {
        // ROM ultra-simple : affiche un pixel au centre et boucle
        return new Uint8Array([
            0x60, 0x20, // V0 = 32 (centre X) [0x200]
            0x61, 0x10, // V1 = 16 (centre Y) [0x202]
            0xA2, 0x0A, // I = 0x20A (sprite) [0x204]
            0xD0, 0x11, // Dessiner 1 pixel [0x206]
            0x12, 0x08, // Boucle infinie à 0x208 [0x208]
            0x80 // Sprite: 1 pixel (10000000) [0x20A]
        ]);
    }

    createBlinkROM() {
        // ROM clignotante CORRIGÉE - pixel qui apparaît/disparaît
        return new Uint8Array([
            0x60, 0x20, // V0 = 32 (centre X) [0x200]
            0x61, 0x10, // V1 = 16 (centre Y) [0x202]
            0xA2, 0x16, // I = 0x216 (sprite) [0x204]

            // Boucle principale [0x206]
            0x00, 0xE0, // CLS - Effacer l'écran [0x206]
            0xD0, 0x11, // DRW - Dessiner 1 pixel [0x208]

            // Délai [0x20A]
            0x62, 0x20, // V2 = 32 (compteur) [0x20A]
            0x72, 0xFF, // V2 = V2 - 1 [0x20C]
            0x32, 0x00, // Skip si V2 == 0 [0x20E]
            0x12, 0x0C, // Jump à 0x20C (délai) [0x210]

            0x12, 0x06, // Jump à 0x206 (recommencer) [0x212]

            0x00, 0x00, // Padding [0x214]
            0x80 // Sprite : un seul pixel [0x216]
        ]);
    }

    createTestROM() {
        // ROM qui affiche un carré et le fait bouger
        return new Uint8Array([
            0x60, 0x10, // V0 = 16 (position X) [0x200]
            0x61, 0x08, // V1 = 8 (position Y) [0x202] 
            0xA2, 0x14, // I = 0x214 (adresse du sprite) [0x204]

            // Boucle principale [0x206]
            0x00, 0xE0, // CLS [0x206]
            0xD0, 0x14, // Dessiner sprite 8x4 [0x208]
            0x70, 0x01, // V0 = V0 + 1 (bouger X) [0x20A]
            0x30, 0x40, // Skip si V0 == 64 [0x20C]
            0x12, 0x06, // Jump boucle [0x20E]
            0x60, 0x00, // V0 = 0 (reset X) [0x210]
            0x12, 0x06, // Jump boucle [0x212]

            // Sprite : un carré 8x4 [0x214]
            0xFF, 0x81, 0x81, 0xFF
        ]);
    }

    createDemoROM() {
        // ROM démo : utilise les fonts Chip-8 pour afficher "BE" (B=0xB, E=0xE)
        return new Uint8Array([
            // Effacer l'écran d'abord
            0x00, 0xE0,   // CLS [0x200]
            
            // Afficher 'B' (0xB)
            0x60, 0x10, // V0 = 16 (pos X pour B) [0x202]
            0x61, 0x0C, // V1 = 12 (pos Y) [0x204]
            0x62, 0x0B, // V2 = 0xB (caractère B) [0x206]
            0xF2, 0x29, // I = font address de V2 [0x208]
            0xD0, 0x15, // Dessiner B (5 lignes) [0x20A]

            // Afficher 'E' (0xE)
            0x60, 0x1C, // V0 = 28 (pos X pour E) [0x20C]
            0x62, 0x0E, // V2 = 0xE (caractère E) [0x20E]
            0xF2, 0x29, // I = font address de V2 [0x210]
            0xD0, 0x15, // Dessiner E (5 lignes) [0x212]

            // Boucle infinie
            0x12, 0x14, // Jump à cette instruction [0x214]
        ]);
    }

    // ========== DEBUG ET MONITORING ==========

    updateDebugInfo() {
        if (!this.emulator) return;

        try {
            const debugInfo = this.emulator.get_debug_info();
            document.getElementById('registers-info').textContent = debugInfo;

            // Vérifier l'état de santé
            const isHealthy = debugInfo.includes('Err: 0');
            if (!isHealthy && this.running) {
                console.warn('⚠️ CPU en état dégradé:', debugInfo);
            }

        } catch (error) {
            console.error('❌ Erreur debug info:', error);
        }
    }

    memoryDump() {
        if (!this.emulator) {
            this.showError('Émulateur non initialisé');
            return;
        }

        try {
            console.log('🗃️ Memory dump demandé');
            // TODO: Implémenter dump mémoire via une fonction Rust exposée
            document.getElementById('memory-info').textContent =
                'Memory dump: Fonction à implémenter côté Rust';
        } catch (error) {
            this.handleError('Erreur memory dump', error);
        }
    }

    // ========== INTERFACE UTILISATEUR ==========

    updateStatus(status) {
        document.getElementById('status').textContent = status;
    }

    hideLoading() {
        this.loadingEl.classList.add('hidden');
        setTimeout(() => {
            this.loadingEl.style.display = 'none';
        }, 500);
    }

    showError(message) {
        console.error('🚨', message);
        alert('❌ ' + message);
        this.updateStatus('❌ Erreur');
    }

    handleError(type, error) {
        this.errorCount++;
        this.lastError = { type, error, timestamp: new Date() };

        console.error(`🚨 ${type}:`, error);

        // Arrêter l'émulateur en cas d'erreur grave
        if (this.errorCount > 3) {
            this.stop();
            this.showError(`${type}: ${error.message || error}`);
        } else {
            this.updateStatus(`⚠️ ${type}`);
        }
    }

    // ========== STATISTIQUES ==========

    getStats() {
        return {
            fps: this.fps,
            speed: this.actualSpeed,
            errors: this.errorCount,
            running: this.running,
            emulator: this.emulator ? 'OK' : 'NULL'
        };
    }

    logStats() {
        const stats = this.getStats();
        console.table(stats);
    }
}

// ========== INITIALISATION ==========

// Initialisation de l'application
const app = new Ferris8App();

// Démarrage
app.init().catch(error => {
    console.error('💥 Échec d\'initialisation:', error);
    document.getElementById('loading').innerHTML = `
        <div style="color: red; text-align: center;">
            <h2>❌ Erreur de chargement</h2>
            <p>${error.message}</p>
            <p>Vérifiez la console pour plus de détails</p>
        </div>
    `;
});

// Exposition globale pour debug
window.ferris8 = app;
window.ferris8Debug = () => app.logStats();

// Service worker pour les performances (optionnel)
if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('./sw.js').catch(console.warn);
}

console.log('🦀 Ferris-8 JavaScript module chargé avec succès');