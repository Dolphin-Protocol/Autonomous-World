# Autonomous World Game 🎮

A Rust-based autonomous world game built with WASM, featuring blockchain integration and multiplayer monopoly gameplay.

## 🌟 Features

- **Cross-Platform Compatibility**: Play on both mobile and web browsers
- **Blockchain Integration**: Connect your wallet and interact with SUI testnet
- **Multiplayer Monopoly**: Challenge your friends in our integrated monopoly game
- **Autonomous World**: Explore and interact in a persistent game environment

## 🚀 Getting Started

### First Time Setup
⚠️ **Important**: The game may take **30-40 seconds** to load on first login. Please be patient!

### How to Play

#### Web Controls
- **Arrow Keys**: Move your character around the world
- **Spacebar**: Open/close doors and interact with objects
- **Mouse Click**: Alternative movement by clicking on the screen

#### Mobile Controls
- **Touch**: Tap on the screen to move your character

### Wallet Connection
🔗 **Always connect your wallet first** before starting any game interactions!

### Accessing Monopoly Game
1. Navigate your character close to the play table
2. Press **Spacebar** when near the table
3. Pay **0.005 SUI** on testnet to enter
4. Get redirected to the monopoly game
5. Invite friends to play with you!

## 🎯 Game Links

- **Main Autonomous World**: https://dolphin-protocol.github.io/Autonomous-World/
- **Monopoly Game**: https://monopoly-frontend-git-main-badukweis-projects.vercel.app/

## 🛠️ Technical Details

### Built With
- **Rust** - Core game logic
- **WASM** - Web Assembly target for browser compatibility
- **Macroquad** - Rust game development framework
- **SUI Blockchain** - Testnet integration for transactions

### Architecture
This project compiles Rust code to WebAssembly (WASM) to run efficiently in web browsers while maintaining the performance benefits of Rust.

## 🎮 Gameplay Instructions

1. **Launch the Game**: Open the autonomous world link
2. **Wait for Loading**: Allow 30-40 seconds for initial load
3. **Connect Wallet**: Essential for blockchain interactions
4. **Explore**: Use arrow keys or click to move around
5. **Find the Play Table**: Look for the monopoly table in the world
6. **Enter Monopoly**: Press spacebar near the table and pay 0.005 SUI
7. **Play with Friends**: Share the monopoly link with friends to join

## ⚠️ Known Limitations

- Key bindings for interactions are limited due to development time constraints
- Mobile interaction primarily relies on touch controls
- First load may be slow due to WASM compilation

## 🙏 Acknowledgments

This project is highly inspired by **raylib** from the Go ecosystem and built using the amazing **macroquad** framework. 

Special thanks to **not-fl3** for the incredible work on macroquad that made this project possible!

## 🔧 Development

### branch
- The latest source code is all at feat/wasm-bindgen

### Prerequisites
- Rust toolchain
- WASM target: `rustup target add wasm32-unknown-unknown`
- Cargo make (optional but recommended)

### Building
```bash
# While setup requirea a bit complicated actions, for the detailed, please checkout the scripts.sh
bash ./scripts.sh autonomous-game --release
```

## 📱 Platform Support

- ✅ Web Browsers (Chrome, Firefox, Safari, Edge)
- ✅ Mobile Browsers (iOS Safari, Android Chrome)
- ✅ Desktop Web Applications

## 🌐 Blockchain Integration

- **Network**: SUI Testnet
- **Transaction Fee**: 0.005 SUI for monopoly game entry
- **Wallet Support**: Compatible with SUI wallet extensions

---

**Ready to explore the autonomous world? Connect your wallet and start your adventure!** 🚀
