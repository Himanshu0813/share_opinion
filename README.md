# share_opinion

Project Title: Share Opinion

Description: 
This is blockchain based platfrom where use can write their opinion and post their opinion directly and anonymously onto the blockchain. And anyone can see that post.

.
├── src
│   ├── lib.rs          # Main contract implementation
│   └── types.rs        # Custom types used in the contract
├── Cargo.toml          # Project dependencies and metadata
└── README.md           # Project documentation

git clone <repository-url>
cd <project-directory>

cargo build

cargo test

# Assuming you have the necessary tools and configurations set up
soroban deploy --wasm target/wasm32-unknown-unknown/release/share_opinion.wasm

# Example CLI interaction
soroban invoke --wasm target/wasm32-unknown-unknown/release/share_opinion.wasm --id <contract-id> --fn post_opinion --arg <opinion_msg>



