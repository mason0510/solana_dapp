#!/bin/sh
i=0
for line in `cat wallet_list.txt`
do
	##--bridge-contract-pid 9st7EdZS5GgDjjKVFk7gR62Smp1YtirhdNCoQax7mxJP --receiver-wallet 9EZZmeAE16RsKPxbL9VXBjGFooPsKfePRxfyLJrp8umu --token-address fAKA84inpr8yi4UoQ7Bu7PWsuSK8WaJiz5saBZiL286
	((i++))
	echo "request airdrop for $line at line $i"
	for ((i=0;i<=15;i++)); do 
	proxychains4 solana airdrop 2 $line;
	done;
	proxychains4 solana  balance $line
	##proxychains4 solana transfer 6iytHt6hJ9szSvNVL713JoioXPLfoPGjKKTSCUhUtH73 20 --allow-unfunded-recipient
	proxychains4 solana transfer 6iytHt6hJ9szSvNVL713JoioXPLfoPGjKKTSCUhUtH73 20 --keypair $i.json
	proxychains4 solana  balance $line
done
