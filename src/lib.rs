
pub mod natural;
pub mod object;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// Number exceeded 32 bits
    NaturalOverflow,
    /// Non-'case' nodes may not have hidden children
    NonCaseHiddenChild,
    /// 'case' nodes may have at most one hidden child
    CaseMultipleHiddenChildren,
    /// Bitstream ended early
    EndOfStream,
    /// Unrecognized node
    ParseError,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Iden,
    Unit,
    InjL(usize),
    InjR(usize),
    Take(usize),
    Drop(usize),
    Comp(usize, usize),
    Case(usize, usize),
    Pair(usize, usize),
    Disconnect(usize, usize),
    Witness(Vec<u8>),
    Fail([u8; 32], [u8; 32]),
    Hidden([u8; 32]),
}

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper.
pub fn decode_node_no_witness<BitStream: Iterator<Item = bool>>(
    mut iter: BitStream,
) -> Result<Node, Error> {
    let b1 = match iter.next() {
        Some(bit) => bit,
        None => return Err(Error::EndOfStream),
    };
    let b2 = match iter.next() {
        Some(bit) => bit,
        None => return Err(Error::EndOfStream),
    };
    let b3 = match iter.next() {
        Some(bit) => bit,
        None => return Err(Error::EndOfStream),
    };
    let b4 = match iter.next() {
        Some(bit) => bit,
        None => return Err(Error::EndOfStream),
    };
    match (b1, b2, b3, b4) {
        (false, true, true, false) => {
            let mut h = [0; 32];
            for i in 0..32 {
                for b in 0..8 {
                    match iter.next() {
                        Some(true) => h[i] |= 1 << (7 - b),
                        Some(false) => {},
                        None => return Err(Error::EndOfStream),
                    };
                }
            }
            Ok(Node::Hidden(h))
        },
        (false, true, true, true) => {
            Ok(Node::Witness(vec![]))
        },
        _ => {
            let b5 = match iter.next() {
                Some(bit) => bit,
                None => return Err(Error::EndOfStream),
            };
            match (b1, b2, b3, b4, b5) {
                (false, false, false, false, false) => {
                    Ok(Node::Comp(
                        natural::decode(&mut iter)?,
                        natural::decode(&mut iter)?,
                    ))
                },
                (false, false, false, false, true) => {
                    Ok(Node::Case(
                        natural::decode(&mut iter)?,
                        natural::decode(&mut iter)?,
                    ))
                },
                (false, false, false, true, false) => {
                    Ok(Node::Pair(
                        natural::decode(&mut iter)?,
                        natural::decode(&mut iter)?,
                    ))
                },
                (false, false, false, true, true) => {
                    Ok(Node::Disconnect(
                        natural::decode(&mut iter)?,
                        natural::decode(&mut iter)?,
                    ))
                },
                (false, false, true, false, false)
                    => Ok(Node::InjL(natural::decode(&mut iter)?)),
                (false, false, true, false, true)
                    => Ok(Node::InjR(natural::decode(&mut iter)?)),
                (false, false, true, true, false)
                    => Ok(Node::Take(natural::decode(&mut iter)?)),
                (false, false, true, true, true)
                    => Ok(Node::Drop(natural::decode(&mut iter)?)),
                (false, true, false, false, false) => Ok(Node::Iden),
                (false, true, false, false, true) => Ok(Node::Unit),
                (false, true, false, true, false) => {
                    let mut h1 = [0; 32];
                    let mut h2 = [0; 32];
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h1[i] |= 1 << (7 - b),
                                Some(false) => {},
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h2[i] |= 1 << (7 - b),
                                Some(false) => {},
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    Ok(Node::Fail(h1, h2))
                },
                (false, true, false, true, true) => Err(Error::ParseError),
                (false, true, false, false, true) => Err(Error::ParseError),
                (true, _, _, _, _) => Err(Error::ParseError),
                (false, true, true, _, _) => unreachable!(),
            }
        },
    }
}
