use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::{Keypair, Signer}, native_token::LAMPORTS_PER_SOL};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    generate_keypair_and_print();
    load_keypair_and_print();
    check_blance_and_print();
}

fn generate_keypair_and_print() {
    let keypair: Keypair = Keypair::new(); 
    let public = keypair.pubkey();
    let private = keypair.to_bytes();
    println!("Public: {public}");
    // println!("Private: {:?}", private);
}

fn load_keypair_and_print() {
    let key_pair = get_keypair_from_env();
    println!("Public: {}", key_pair.pubkey());
}

fn get_keypair_from_env() -> Keypair {
    let key_string = std::env::var("SECRET_KEY").expect("SECRET_KEY value must be set");
    let bytes_array: Vec<u8> = serde_json::from_str(key_string.as_str()).expect("Failed to parse");
    return Keypair::from_bytes(&bytes_array).expect("Failed to construct the Keypair object");
}

fn check_blance_and_print() {
    let devnet_url = "https://api.devnet.solana.com";
    let keypair = get_keypair_from_env();

    let rpc_client = RpcClient::new(devnet_url);
    let balance = rpc_client.get_account(&keypair.pubkey()).unwrap();

    println!("{} sol for {:?}",balance.lamports / LAMPORTS_PER_SOL, keypair.pubkey()); 
}