#!/bin/bash

# 🧪 Script de test et validation pour Ferris-8

set -e

echo "🧪 ===== TESTS DE VALIDATION FERRIS-8 ===== "
echo ""

# Couleurs
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

test_pass() { echo -e "✅ ${GREEN}$1${NC}"; }
test_fail() { echo -e "❌ ${RED}$1${NC}"; }
test_info() { echo -e "ℹ️  $1"; }

# Test 1: Structure des fichiers
test_info "Test 1: Vérification de la structure des fichiers"

required_files=(
    "Cargo.toml"
    "src/lib.rs"
    "src/cpu.rs"
    "src/memory.rs"
    "src/display.rs"
    "src/input.rs"
    "src/audio.rs"
    "web/index.html"
    "web/main.js"
    "web/style.css"
    "launch.sh"
)

missing_files=()
for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        missing_files+=("$file")
    fi
done

if [ ${#missing_files[@]} -eq 0 ]; then
    test_pass "Tous les fichiers requis sont présents"
else
    test_fail "Fichiers manquants: ${missing_files[*]}"
    echo "   Crée ces fichiers avant de continuer"
    exit 1
fi

# Test 2: Compilation Rust
test_info "Test 2: Compilation du code Rust"
if cargo check --quiet; then
    test_pass "Code Rust valide"
else
    test_fail "Erreurs de compilation Rust"
    exit 1
fi

# Test 3: Target WebAssembly
test_info "Test 3: Target WebAssembly"
if rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    test_pass "Target WebAssembly installée"
else
    test_fail "Target WebAssembly manquante"
    echo "   Installe avec: rustup target add wasm32-unknown-unknown"
    exit 1
fi

# Test 4: wasm-pack
test_info "Test 4: Disponibilité de wasm-pack"
if command -v wasm-pack &> /dev/null; then
    wasm_pack_version=$(wasm-pack --version | cut -d' ' -f2)
    test_pass "wasm-pack disponible (version $wasm_pack_version)"
else
    test_fail "wasm-pack manquant"
    exit 1
fi

# Test 5: Compilation WebAssembly (test rapide)
test_info "Test 5: Compilation WebAssembly de test"
if wasm-pack build --target web --out-dir test-pkg --dev --quiet; then
    test_pass "Compilation WebAssembly réussie"
    rm -rf test-pkg/
else
    test_fail "Échec compilation WebAssembly"
    exit 1
fi

# Test 6: Syntaxe JavaScript
test_info "Test 6: Validation JavaScript"
if command -v node &> /dev/null; then
    if node -c web/main.js 2>/dev/null; then
        test_pass "Syntaxe JavaScript valide"
    else
        test_fail "Erreurs de syntaxe JavaScript"
        exit 1
    fi
else
    test_info "Node.js non trouvé, validation JS ignorée"
fi

# Test 7: Taille des fichiers web
test_info "Test 7: Vérification des fichiers web"
html_size=$(wc -c < web/index.html)
css_size=$(wc -c < web/style.css)
js_size=$(wc -c < web/main.js)

if [ $html_size -gt 1000 ] && [ $css_size -gt 1000 ] && [ $js_size -gt 5000 ]; then
    test_pass "Fichiers web de taille appropriée"
    echo "   HTML: ${html_size} bytes, CSS: ${css_size} bytes, JS: ${js_size} bytes"
else
    test_fail "Fichiers web trop petits (possiblement vides)"
    echo "   HTML: ${html_size} bytes, CSS: ${css_size} bytes, JS: ${js_size} bytes"
    exit 1
fi

# Test 8: Permissions d'exécution
test_info "Test 8: Permissions des scripts"
if [ -x "launch.sh" ]; then
    test_pass "launch.sh exécutable"
else
    test_fail "launch.sh non exécutable"
    echo "   Corrige avec: chmod +x launch.sh"
    exit 1
fi

# Test 9: Configuration Cargo.toml
test_info "Test 9: Validation Cargo.toml"
if grep -q 'crate-type.*cdylib' Cargo.toml && grep -q 'wasm-bindgen' Cargo.toml; then
    test_pass "Configuration WebAssembly correcte dans Cargo.toml"
else
    test_fail "Configuration WebAssembly manquante dans Cargo.toml"
    exit 1
fi

echo ""
echo "🎉 ===== TOUS LES TESTS PASSÉS ===== "
echo ""
echo "✅ Ferris-8 est prêt à être lancé !"
echo ""
echo "🚀 Prochaines étapes:"
echo "   1. Lance: ./launch.sh"
echo "   2. Ouvre http://localhost:8000"
echo "   3. Teste avec '💫 Charger démo'"
echo ""
echo "🐛 Si des problèmes surviennent:"
echo "   • Vérifie la console du navigateur (F12)"
echo "   • Lance: ./launch.sh --clean"
echo "   • Recompile avec: ./test.sh && ./launch.sh"
echo ""