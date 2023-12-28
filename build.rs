pub fn main() {
    uniffi::generate_scaffolding("src/users.udl").unwrap();
}
