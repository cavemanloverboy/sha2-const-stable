//! This demonstrates how to use sha2-const-stable to easily
//! obtain anchor instruction discriminators at compile time.
use sha2_const_stable::Sha256;

pub const fn anchor_discriminator(preimage: &'static str) -> u64 {
    let hash: [u8; 32] = Sha256::new().update(preimage.as_bytes()).finalize();
    u64::from_le_bytes([
        hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7],
    ])
}

#[repr(u64)]
/// We can collect any subset of instructions within an enum
pub enum SomeAnchorProgramInstruction {
    // This takes in a pubkey which will be the member assigned
    Initialize = anchor_discriminator("global:initialize"),

    // This executes a transaction and takes in 2 accounts and lamports
    Execute = anchor_discriminator("global:execute"),
}

// Then we can create some helper to initialize the ix data needed
impl SomeAnchorProgramInstruction {
    pub fn init(member: &[u8; 32]) -> [u8; 40] {
        let mut data = [0; 40];

        // Write discrmininator
        data[..8].copy_from_slice(&(SomeAnchorProgramInstruction::Initialize as u64).to_le_bytes());

        // Write 32 public key bytes
        data[8..40].copy_from_slice(member.as_ref());

        data
    }

    pub fn execute(from: &[u8; 32], to: &[u8; 32], lamports: u64) -> [u8; 80] {
        let mut data = [0; 80];

        // Write discrmininator
        data[..8].copy_from_slice(&(SomeAnchorProgramInstruction::Execute as u64).to_le_bytes());

        // Write two accounts
        data[8..40].copy_from_slice(from.as_ref());
        data[40..72].copy_from_slice(to.as_ref());

        // Write lamports
        data[72..].copy_from_slice(&lamports.to_le_bytes());

        data
    }
}

fn main() {
    let member = [0; 32];
    println!(
        "Initialize ix data: {:?}",
        SomeAnchorProgramInstruction::init(&member)
    );

    let from = [7; 32];
    let to = [25; 32];
    let lamports = u64::MAX;
    println!(
        "Execute ix data: {:?}",
        SomeAnchorProgramInstruction::execute(&from, &to, lamports),
    );
}
