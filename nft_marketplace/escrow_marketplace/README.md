## build & deploy
=
```
proxychains4 anchor build
```

## test
```
proxychains4 anchor test --skip-build --skip-deploy
//or
(cd cli;proxychains4 cargo run -- --bridge-contract-pid 9HiRJw3dYo2MV9B1WrqFfoNjWRPS19mjVDCPqAxuMPfb  --receiver-wallet 677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa  --token-address 7YYNXbfwd5i5scpez18fTkEh2MRHJXoMHPffnWNcpFYf);
```

## todo
- [X] 1、support multi coin
- [X] 2、support sol pay
- 3、more error code
- 4、fix typescript testcase
- [X] 5、add global config state

