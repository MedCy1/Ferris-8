#!/bin/bash

# 🦀 Ferris-8 - Script de lancement pour l'émulateur Chip-8

set -e  # Arrêter en cas d'erreur

echo "==============================================="
echo "    🦀 FERRIS-8 - Émulateur Chip-8 moderne"
echo "    Rust + WebAssembly + Interface moderne"
echo "==============================================="
echo ""

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}ℹ️  $1${NC}"; }
log_success() { echo -e "${GREEN}✅ $1${NC}"; }
log_warning() { echo -e "${YELLOW}⚠️  $1${NC}"; }
log_error() { echo -e "${RED}❌ $1${NC}"; }

# Vérifier qu'on est dans le bon dossier
if [ ! -f "Cargo.toml" ]; then
    log_error "Lance ce script depuis le dossier ferris8/ qui contient Cargo.toml"
    echo "   Usage: ./launch.sh [--clean]"
    exit 1
fi

# Option de nettoyage
CLEAN_BUILD=false
if [ "$1" = "--clean" ]; then
    CLEAN_BUILD=true
    log_info "Mode nettoyage activé"
fi

# Vérifier les dépendances OBLIGATOIRES
log_info "Vérification des dépendances..."

if ! command -v cargo &> /dev/null; then
    log_error "Rust/Cargo n'est pas installé!"
    echo "   Installe avec: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

if ! command -v wasm-pack &> /dev/null; then
    log_error "wasm-pack n'est pas installé!"
    echo "   Installe avec: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Vérifier la target WebAssembly
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    log_warning "Target WebAssembly manquante, installation..."
    rustup target add wasm32-unknown-unknown
fi

# Vérifier Python ou serveur HTTP
PYTHON_CMD=""
if command -v python3 &> /dev/null; then
    PYTHON_CMD="python3 -m http.server"
elif command -v python &> /dev/null; then
    PYTHON_CMD="python -m SimpleHTTPServer"
elif command -v node &> /dev/null && command -v npx &> /dev/null; then
    PYTHON_CMD="npx serve -p"
else
    log_error "Aucun serveur HTTP trouvé!"
    echo "   Installe Python 3 ou Node.js pour continuer"
    exit 1
fi

log_success "Dépendances vérifiées"

# Nettoyage conditionnel
if [ "$CLEAN_BUILD" = true ]; then
    log_info "🧹 Nettoyage complet..."
    rm -rf pkg/ web/pkg/ target/wasm32-unknown-unknown/
    cargo clean
    log_success "Nettoyage terminé"
else
    log_info "🧹 Nettoyage léger..."
    rm -rf pkg/ web/pkg/
fi

echo ""

# Vérification du code Rust AVANT compilation
log_info "🔍 Vérification du code Rust..."
if ! cargo check --target wasm32-unknown-unknown --quiet; then
    log_error "Erreurs dans le code Rust, impossible de continuer"
    echo ""
    echo "🔧 Solutions possibles:"
    echo "   1. Corrige les erreurs de compilation ci-dessus"
    echo "   2. Vérifie que tous les fichiers src/ existent"
    echo "   3. Lance: ./launch.sh --clean"
    exit 1
fi
log_success "Code Rust valide"

# Compilation WebAssembly avec gestion d'erreurs
echo ""
log_info "🔧 Compilation du module WebAssembly..."
echo "   (Première compilation peut prendre 2-3 minutes)"

# Build avec options optimisées
if ! wasm-pack build --target web --out-dir pkg --release; then
    log_error "Échec de la compilation WebAssembly"
    echo ""
    echo "🔧 Solutions de dépannage:"
    echo "   1. Lance: ./launch.sh --clean"
    echo "   2. Vérifie que wasm-pack est à jour: cargo install wasm-pack"
    echo "   3. Vérifie les erreurs ci-dessus"
    exit 1
fi

log_success "Compilation WebAssembly réussie!"

# Vérifier que les fichiers WASM sont bien générés
if [ ! -f "pkg/ferris8.js" ] || [ ! -f "pkg/ferris8_bg.wasm" ]; then
    log_error "Fichiers WebAssembly manquants dans pkg/"
    ls -la pkg/ || echo "Le dossier pkg/ n'existe pas"
    exit 1
fi

# Vérifier les fichiers web
echo ""
log_info "📁 Vérification des fichiers web..."

if [ ! -f "web/index.html" ]; then
    log_error "web/index.html manquant!"
    echo "   Assure-toi d'avoir créé tous les fichiers dans web/"
    exit 1
fi

if [ ! -f "web/style.css" ]; then
    log_warning "web/style.css manquant"
fi

if [ ! -f "web/main.js" ]; then
    log_error "web/main.js manquant!"
    exit 1
fi

log_success "Fichiers web trouvés"

# Copier les fichiers WASM dans le dossier web avec vérification
log_info "📦 Copie des fichiers WebAssembly vers web/..."
mkdir -p web/pkg
cp -f pkg/* web/pkg/

# Vérifier la copie
if [ ! -f "web/pkg/ferris8.js" ]; then
    log_error "Échec de la copie des fichiers WASM"
    exit 1
fi

log_success "Fichiers WASM copiés"

# Statistiques du build
echo ""
log_info "📊 Statistiques du build:"
wasm_size=$(du -h pkg/ferris8_bg.wasm | cut -f1)
js_size=$(du -h pkg/ferris8.js | cut -f1)
total_files=$(ls pkg/ | wc -l)
echo "   • Module WASM: $wasm_size"
echo "   • Bindings JS: $js_size"
echo "   • Fichiers: $total_files dans pkg/"

# Instructions détaillées
echo ""
echo "🎮 ===== INSTRUCTIONS D'UTILISATION ===== "
echo ""
echo "📍 URL du serveur: http://localhost:8000"
echo "📂 Dossier servi:  $(pwd)/web"
echo ""
echo "🎯 TESTS À EFFECTUER:"
echo "   1. ✅ La page se charge sans erreur"
echo "   2. ✅ Clique sur '💫 Charger démo'"
echo "   3. ✅ Clique sur '▶️ Start'"
echo "   4. ✅ Un pixel blanc doit clignoter au centre"
echo "   5. ✅ Teste le clavier (voir mapping ci-dessous)"
echo ""
echo "⌨️  MAPPING CLAVIER:"
echo "   Chip-8:  1 2 3 C     Ton clavier: 1 2 3 4"
echo "           4 5 6 D         →        Q W E R"
echo "           7 8 9 E                  A S D F"
echo "           A 0 B F                  Z X C V"
echo ""
echo "🔧 CONTRÔLES:"
echo "   • ▶️ Start / ⏹️ Stop / 🔄 Reset"
echo "   • 📁 Glisse-dépose des fichiers .ch8/.c8"
echo "   • 🎚️ Ajuste vitesse avec le slider"
echo "   • 🔍 Debug info en temps réel"
echo ""
echo "🐛 DÉPANNAGE:"
echo "   • Ouvre DevTools (F12) pour voir les erreurs"
echo "   • Console → onglet Network pour voir les requêtes"
echo "   • Si problème: Ctrl+C puis ./launch.sh --clean"
echo ""
echo "⚠️  Appuie sur Ctrl+C pour arrêter le serveur"
echo ""

# Fonction de nettoyage au Ctrl+C
cleanup() {
    echo ""
    log_info "Arrêt du serveur..."
    exit 0
}
trap cleanup SIGINT SIGTERM

# Déterminer le port (8000 par défaut, 8001 si occupé)
PORT=8000
if command -v lsof &> /dev/null && lsof -Pi :8000 -sTCP:LISTEN -t >/dev/null 2>&1; then
    log_warning "Port 8000 occupé, utilisation du port 8001"
    PORT=8001
fi

# Essayer d'ouvrir le navigateur automatiquement
if command -v xdg-open &> /dev/null; then
    xdg-open "http://localhost:$PORT" 2>/dev/null &
elif command -v open &> /dev/null; then
    open "http://localhost:$PORT" 2>/dev/null &
fi

# Démarrer le serveur depuis le dossier web
log_success "🌐 Serveur démarré sur http://localhost:$PORT"
echo ""

cd web || exit 1

# Démarrer le serveur approprié
if echo "$PYTHON_CMD" | grep -q "python3"; then
    python3 -m http.server $PORT
elif echo "$PYTHON_CMD" | grep -q "python "; then
    python -m SimpleHTTPServer $PORT
else
    npx serve -p $PORT
fi