#!/bin/sh
for line in `cat nft_address2.txt`
do
	##--bridge-contract-pid 9st7EdZS5GgDjjKVFk7gR62Smp1YtirhdNCoQax7mxJP --receiver-wallet 9EZZmeAE16RsKPxbL9VXBjGFooPsKfePRxfyLJrp8umu --token-address fAKA84inpr8yi4UoQ7Bu7PWsuSK8WaJiz5saBZiL286
	echo "transfer $line"
    proxychains4 cargo run -- --bridge-contract-pid 9st7EdZS5GgDjjKVFk7gR62Smp1YtirhdNCoQax7mxJP --receiver-wallet 9EZZmeAE16RsKPxbL9VXBjGFooPsKfePRxfyLJrp8umu --token-address ${line} 
done
