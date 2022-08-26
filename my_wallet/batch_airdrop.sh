#!/bin/sh
line=0
for wallet in `cat wallet_list.txt`
do
	##--bridge-contract-pid 9st7EdZS5GgDjjKVFk7gR62Smp1YtirhdNCoQax7mxJP --receiver-wallet 9EZZmeAE16RsKPxbL9VXBjGFooPsKfePRxfyLJrp8umu --token-address fAKA84inpr8yi4UoQ7Bu7PWsuSK8WaJiz5saBZiL286
	((line++))
	echo "request airdrop for $wallet at line $line"
	for ((i=0;i<=12;i++)); do 
	solana airdrop 2 $wallet;
	done;
	solana  balance $wallet
	##proxychains4 solana transfer 6iytHt6hJ9szSvNVL713JoioXPLfoPGjKKTSCUhUtH73 20 --allow-unfunded-recipient
	solana transfer 6iytHt6hJ9szSvNVL713JoioXPLfoPGjKKTSCUhUtH73 20 --keypair $line.json
	solana  balance $wallet
done
