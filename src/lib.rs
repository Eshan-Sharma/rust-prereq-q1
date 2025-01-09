#[cfg(test)]
mod tests {
    use std::io::{self, BufRead};

    use solana_sdk::{self, bs58};
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    };

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana Wallet:{}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file");
        println!("{:?}", kp.to_bytes());
    }
    //FAuoWgF5aGC33CyWnM9XXF9rGognWYEBqLsd4ffa8eq3

    #[test]
    fn airdrop() {}

    #[test]
    fn transfer_sol() {}

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        print!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }
}
