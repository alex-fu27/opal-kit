use argon2::Argon2;
use block_buffer::Eager;
use digest::core_api::*;
use digest::HashMarker;
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use sha2::Sha512;
use typenum::consts::U256;
use typenum::{IsLess, Le, NonZero};

fn sedutil_hash<D>(password: &[u8], salt: &[u8]) -> Vec<u8>
where
    D: CoreProxy,
    D::Core: Sync
        + HashMarker
        + UpdateCore
        + FixedOutputCore
        + BufferKindUser<BufferKind = Eager>
        + Default
        + Clone,
    <D::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<D::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    const ROUNDS: u32 = 75000;
    const HASHSIZE: usize = 32;

    if password.len() == 0 {
        return vec![];
    }

    let mut out = vec![b' '; HASHSIZE];

    pbkdf2_hmac::<D>(password, salt, ROUNDS, &mut out);

    return out;
}

// used by github forks Drive-Trust-Alliance, fabiogermann, ...
fn dta_sedutil_hash(password: &[u8], salt: &[u8]) -> Vec<u8> {
    sedutil_hash::<Sha1>(password, salt)
}

// used by github forks ladar, ...
fn ladar_sedutil_hash(password: &[u8], salt: &[u8]) -> Vec<u8> {
    sedutil_hash::<Sha512>(password, salt)
}

fn hash(password: &[u8], salt: &[u8]) -> Vec<u8> {
    if password.len() == 0 {
        return vec![];
    }

    let mut out = vec![0u8; 32];
    Argon2::default()
        .hash_password_into(password, salt, &mut out)
        .unwrap();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn sedutil_compat() {
        let reference: [u8; 32] =
            hex!("44b9ad82479810effb78a1d7790da89c3566d8ad72806f21efa78a452168bc5b");
        let password: [u8; 4] = [b'1', b'2', b'3', b'4'];
        let serno: [u8; 20] = [
            50, 50, 52, 49, 51, 67, 49, 65, 55, 57, 49, 66, 32, 32, 32, 32, 32, 32, 32, 32,
        ];
        let hashed = dta_sedutil_hash(&password, &serno);
        assert_eq!(hashed, reference);
    }

    #[test]
    fn argon2_hash() {
        let password: [u8; 4] = [b'1', b'2', b'3', b'4'];
        let serno: [u8; 20] = [
            50, 50, 52, 49, 51, 67, 49, 65, 55, 57, 49, 66, 32, 32, 32, 32, 32, 32, 32, 32,
        ];
        let reference: [u8; 32] = [
            232, 8, 203, 28, 46, 232, 29, 155, 250, 104, 240, 26, 19, 157, 159, 202, 76, 162, 67,
            79, 23, 13, 54, 201, 54, 60, 143, 172, 224, 188, 109, 103,
        ];
        let hashed = hash(&password, &serno);
        assert_eq!(hashed, reference);
    }

    #[test]
    fn empty_password_to_empty_hash() {
        let r: Vec<u8> = vec![];
        let serno: [u8; 20] = [
            50, 50, 52, 49, 51, 67, 49, 65, 55, 57, 49, 66, 32, 32, 32, 32, 32, 32, 32, 32,
        ];
        assert_eq!(r, dta_sedutil_hash(&[], &serno));
        assert_eq!(r, ladar_sedutil_hash(&[], &serno));
        assert_eq!(r, hash(&[], &serno));
    }
}
