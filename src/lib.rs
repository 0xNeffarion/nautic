use nautic_dns::protocol::*;

pub fn question() -> Result<(), Box<dyn std::error::Error>> {
    let question = Question::new("beans.opensuse.org".into(), Record::A, 1);

    let bytes: Vec<u8> = question.into();

    for x in bytes.iter() {
        print!("{:08b} ", x);
    }

    println!();
    println!("{:02X?}", bytes);
    println!("{:?}", bytes.len());

    let question = Question::try_from(bytes)?;
    println!("{:?}", question);

    Ok(())
}

pub fn flags() -> Result<(), Box<dyn std::error::Error>> {
    let flags = FlagsBuilder::default()
        .recursion_desired(true)
        .op(OpCode::StandardQuery)
        .message_type(MessageType::Query)
        .build()?;

    let bits: Vec<u8> = flags.into();

    let both_bytes = u16::from_be_bytes([bits[0], bits[1]]);
    println!("{:08b} {:08b}", bits[0], bits[1]);
    println!("{:04x}", both_bytes);

    let flags = Flags::try_from(bits)?;
    println!("{:?}", flags);

    Ok(())
}

pub fn header() -> Result<(), Box<dyn std::error::Error>> {
    let flags = FlagsBuilder::default()
        .recursion_desired(true)
        .op(OpCode::StandardQuery)
        .message_type(MessageType::Query)
        .response(ResponseCode::Refused)
        .build()?;

    let header = HeaderBuilder::default()
        .id(0xa629)
        .flags(flags)
        .questions_size(1)
        .answers_size(0)
        .name_servers_size(0)
        .additional_size(0)
        .build()?;

    let bytes: Vec<u8> = header.into();

    for x in bytes.iter() {
        print!("{:08b} ", x);
    }

    println!();
    println!("{:02x?}", bytes);
    println!("{:?}", bytes.len());

    let header = Header::try_from(bytes)?;
    println!("{:?}", header);

    Ok(())
}
