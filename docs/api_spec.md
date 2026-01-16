# WakeUp Protocol API Specification (v1.0)

## Base URL
`https://wake.550b.cn/api/v1`

## Endpoints

### 1. GET `/challenge`
Retrieves the unique daily phrase that the user must handwrite.
- **Security:** Phrases are unique per wallet and expire after the morning window.

### 2. POST `/verify`
Submit the proof of discipline.
- **Payload:**
  - `image`: Base64 encoded image string.
  - `wallet`: User's Solana Public Key.
- **AI Logic:** Uses Vision Model to extract handwritten text and perform similarity analysis.
- **Anti-Fraud:** Detects if the image is a digital screen or a re-upload of a previous photo.

### 3. GET `/user/stats/{wallet}`
Retrieves the user's historical discipline record.
- **Returns:** Total stakes, success rate, and total "lazy-tax" avoided.
