use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
fn main() {
    let mut times = 0u64;
    loop{
        let key = Keypair::new();
        std::thread::sleep(std::time::Duration::from_secs_f32(0.0000001));
        let mut prefix = key.pubkey().to_string();
        prefix.truncate(4);
        if prefix.eq("KAFU") {
            println!(
                "pubkey {},prikey {:?},prikey_str {}",
                key.pubkey().to_string(),
                key.secret(),
                key.to_base58_string()
            );
        }
        if times % 1000000 == 0 {
            println!("find at {} times",times);
        }
        times += 1;
    }
}