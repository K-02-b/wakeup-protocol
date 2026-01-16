# Technical Architecture

## Overview
WakeUp Protocol is a hybrid system combining decentralized finance (DeFi) with AI Computer Vision.

## Workflow (The "WakeUp" Loop)
1. **Commitment Phase:** User stakes USDC via Solana Smart Contract.
2. **Challenge Generation:** System issues a random daily phrase.
3. **Submission:** User uploads handwritten phrase photo.
4. **AI Verification:** - Backend extracts text using GPT-4o-vision.
   - Text is compared with the challenge phrase.
5. **Settlement:**
   - **Success:** Contract triggers a "Release" or "Reward" function.
   - **Failure:** Contract triggers "Slash" function.

## Security Measures
- **Handwriting Requirement:** Prevents bots from using standard OCR-generated text.
- **Timestamp Check:** Submissions are only accepted within a specific morning window.
