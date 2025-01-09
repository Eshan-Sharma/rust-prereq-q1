#[cfg(test)]
mod tests {
    use std::io::{self, BufRead};

    use solana_client::rpc_client::RpcClient;
    use solana_sdk::signature::read_keypair_file;
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
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("src/dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your txhash:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                )
            }
            Err(e) => println!("Oops, something went wrong: {}", e),
        }
    }
    //https://explorer.solana.com/tx/5zQUegCVYWRm99Y8yvoKECi2ZerH1KHEYRVSmsPgXSfG8r38Gb1nd7qPFX6iaoAJeJwuf1DQLHMjpyQzqQSBRpaj?cluster=devnet

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

    #[test]
    fn wallet_to_base() {
        print!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(",")
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        print!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
