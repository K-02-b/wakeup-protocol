/**
 * WakeUp Protocol - Wallet & UI Shared Logic
 */

let walletKey = null;
let provider = null;

window.addEventListener('pageshow', (event) => {
    if (window.lucide) {
        window.lucide.createIcons();
    }

    window.onclick = function(event) {
        const modal = document.getElementById('walletModal');
        if (event.target == modal) toggleModal();
    }
    
    provider = window.solflare || window.solana;
    
    autoConnect();
    
    if (provider) {
        provider.on('accountChanged', (publicKey) => {
            if (publicKey) {
                console.log("Switched to:", publicKey.toString());
                handleConnection(publicKey.toString());
            } else {
                walletKey = null;
                document.getElementById('walletBtn').innerText = "CONNECT WALLET";
            }
        });
    }
});

async function autoConnect() {
    if (provider) {
        try {
            const resp = await provider.connect({ onlyIfTrusted: true });
            handleConnection(provider.publicKey ? provider.publicKey.toString() : resp.publicKey.toString());
        } catch (err) {
            console.log("Auto-connect skipped.");
        }
    }
}

async function connectWallet() {
    if (walletKey) {
        toggleModal();
        return;
    }

    const btn = document.getElementById('walletBtn');

    if (!provider) {
        alert("Please install Solflare extension!");
        return;
    }

    try {
        btn.innerText = "Connecting...";
        btn.disabled = true; 
        const resp = await provider.connect();
        const address = provider.publicKey ? provider.publicKey.toString() : resp.publicKey.toString();
        handleConnection(address);
    } catch (err) {
        console.error("Connection error:", err);
        alert("Connect failed: " + (err.message || "User rejected"));
        btn.innerText = "CONNECT WALLET";
    } finally {
        btn.disabled = false;
    }
}

async function handleConnection(address) {
    walletKey = address;
    const btn = document.getElementById('walletBtn');
    if (btn) {
        const shortAddress = address.slice(0, 4) + "..." + address.slice(-4);
        btn.innerText = shortAddress;
    }
    
    const modalAddr = document.getElementById('modalAddress');
    if (modalAddr) modalAddr.innerText = address;
    
    updateBalance(address);
}

//  (默认 Devnet，上线改 mainnet-beta)
async function updateBalance(address) {
    const balanceSpan = document.getElementById('modalBalance');
    if (!balanceSpan) return;

    try {
        const connection = new solanaWeb3.Connection(solanaWeb3.clusterApiUrl('devnet'));
        const publicKey = new solanaWeb3.PublicKey(address);
        const balance = await connection.getBalance(publicKey);
        balanceSpan.innerText = (balance / 1000000000).toFixed(4);
    } catch (e) {
        console.error("Fetch balance failed", e);
        balanceSpan.innerText = "---";
    }
}

function toggleModal() {
    const modal = document.getElementById('walletModal');
    if (!modal) return;
    
    const isVisible = modal.style.display === 'flex';
    modal.style.display = isVisible ? 'none' : 'flex';
    
    if (!isVisible && walletKey) updateBalance(walletKey);
}

async function disconnectWallet() {
    const provider = window.solflare || window.solana;
    if (provider) {
        try {
            await provider.disconnect();
            walletKey = null;
            const btn = document.getElementById('walletBtn');
            if (btn) btn.innerText = "CONNECT WALLET";
            toggleModal();
        } catch (e) {
            console.error("Disconnect failed", e);
        }
    }
}