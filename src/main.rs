use bincode::Options;
use nautic_dns::{
    flags::{
        Flags, FlagsBuilder,
        MessageType::{self, Query},
        OpCode::StandardQuery,
        ResponseCode,
    },
    packet::{Question, Record},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let question = Question::new("cloud.neffware.com".into(), Record::AAAA, 1);

    let bits: u64 = question.into();

    println!("{:#064b}", bits);
    println!("{:#20x}", bits);

    let question = Question::try_from(bits)?;
    println!("{:?}", question);

    Ok(())
}
