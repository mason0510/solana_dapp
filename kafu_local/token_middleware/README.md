```
(cd cli;proxychains4 cargo run -- --bridge-contract-pid 9HiRJw3dYo2MV9B1WrqFfoNjWRPS19mjVDCPqAxuMPfb  --receiver-wallet 677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa  --token-address 7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf);
anchor build
anchor deploy
anchor idl parse -f programs/token_middleware/src/lib.rs -o idl/token_middleware_go_v1.json
cargo test
```

