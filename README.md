# BioWallet: Biometric-Wallet for the Unbanked

**BioWallet** - A blockchain-powered community platform built on the **Stellar Network** using **Soroban Smart Contracts**. This project aims to lower the Web3 adoption barrier for unbanked populations by replacing complex private keys with biometric authentication.

---

## 📌 Problem Statement
More than 1.4 billion people worldwide do not have a bank account and struggle to adopt Web3 due to the fear and complexity of managing seed phrases.

Currently, this issue faces several hurdles:
1. **Key Vulnerability:** Users often lose seed phrases, resulting in permanent loss of funds.
2. **Complex UX:** Navigating crypto keys, passwords, and addresses is highly intimidating for beginners.
3. **Lack of Trusted Recovery:** No secure, decentralized backup options exist for non-custodial wallets.

## 🚀 Urgency
Financial exclusion blocks access to modern economic opportunities. A simple, biometric-secured wallet with social recovery lowers the entry barrier for Web3, making secure digital finance accessible to everyone.
* **Lowering Entry Barriers:** Replaces seed phrase headaches with simple fingerprint/face recognition.
* **Secure Social Backup:** Recovers wallets democratically without centralized custody.

## ✨ Key Features
* **Seedless Setup:** Uses WebAuthn/Passkeys (fingerprint or face recognition) instead of seed phrases.
* **Social Recovery:** If a device is lost, access can be recovered democratically through community guardians via Stellar's multisig.
* **Transparent Guardian Management:** Users can register and modify their guardians on-chain.

---

## 🛠 Technical Stack
* **Blockchain:** Stellar Network
* **Smart Contract Engine:** Soroban
* **Language:** Rust
* **Development Environment:** Soroban CLI / Rust toolchain

## 📋 Smart Contract Overview
The contract handles the wallet configurations and social recovery processes:
1. `get_wallet_config()`: Retrieve configurations for registered social recovery accounts.
2. `register_wallet(owner, guardians)`: Register a new wallet config with designated community guardian addresses.
3. `initiate_recovery(owner)`: Initiate social recovery to recover wallet access.

---

## 💡 Future Roadmap
- [ ] **Biometric SDK Integrations:** Provide plug-and-play passkey libraries for Web3 applications.
- [ ] **Guardian Multi-Sig Dashboard:** A portal for guardians to easily vote and sign recovery requests.
- [ ] **Hardware Key Support:** Support physical FIDO2 keys for institutional-level security.

---
## Screenshots
<img width="1920" height="1080" alt="image" src="screenshot.png" />

---
Stellar ID: G3QNBUIWV5HHK73EJF4FO4GWXDEUWO2LEKDV5EAQ7NY6T4GZYSGPJD2S
*Developed for the Stellar Community and the advancement of Decentralized Social Economies.*
