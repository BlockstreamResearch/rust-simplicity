// Simplicity Bindings
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

/// Simplicity error.
#[derive(Debug)]
pub enum Error {
    Malloc,
    BitstreamEOF,
    BitstreamError,
    DataOutOfRange,
    NotYetImplemented,
    DataOutOfOrder,
    FailCode,
    StopCode,
    Hidden,
}

impl Error {
    pub(crate) fn get_result(ret: i32) -> Result<i32, Error> {
        match ret {
            i if i >= 0 => Ok(i),
            -1 => Err(Error::Malloc),
            -2 => Err(Error::BitstreamEOF),
            -3 => Err(Error::BitstreamError),
            -4 => Err(Error::DataOutOfRange),
            -5 => Err(Error::NotYetImplemented),
            -6 => Err(Error::DataOutOfOrder),
            -8 => Err(Error::FailCode),
            -10 => Err(Error::StopCode),
            -12 => Err(Error::Hidden),
            i => panic!("Unexpected error code: {}", i),
        }
    }
}
