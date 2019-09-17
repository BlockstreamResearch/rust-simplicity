
fn get_bit(data: &[u8], idx: usize) -> bool {
    data[idx / 8] & (1 << (idx % 8)) != 0
}

fn set_bit(data: &mut [u8], idx: usize, val: bool) {
    if val {
        data[idx / 8] |= 1 << (idx % 8)
    } else {
        data[idx / 8] &= !(1 << (idx % 8))
    }
}

struct Object {
    data: [u8; 512],
    len: usize,
}

impl Object {
    fn unit() -> Object {
        Object {
            data: [0; 512],
            len: 0,
        }
    }

    fn injl(mut obj: Object) -> Object {
        set_bit(&mut obj.data[..], obj.len, false);
        obj.len += 1;
        obj
    }

    fn injr(mut obj: Object) -> Object {
        set_bit(&mut obj.data[..], obj.len, true);
        obj.len += 1;
        obj
    }

    fn pair(mut obj1: Object, mut obj2: Object) -> Object {
        unimplemented!()
    }
}
