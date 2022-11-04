// we need to send to channel 8 in mailbox 0 to get property tags of the videocore
fn read_from_mailbox(data: u32, channel: u8) -> bool {
    // check if empty flag unitil it is not set
    // read data from register
    // if lower four bits don't match the channel -> failure
    // returned data is in upper 28 bits
    return true
}
