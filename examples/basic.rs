extern crate hashicorp_vault as vault;

fn main() {
    let host = "http://localhost:8200";
    let token = "test12345";
    let client = vault::Client::new(host, token).unwrap();

    let _ = client.set_secret("foo", "bar");

    let sys = client.sys();
    sys.foo();
    let mounts = sys.list_mounts().unwrap();
    println!("Mounts available are: {:#?}", mounts);

    let secret = client.get_secret("foo").unwrap();

    println!("Secret is \"bar\": {}", secret);
}
