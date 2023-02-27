#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memcpy(destination: *mut u8, source: *mut u8, len: usize) -> *mut u8 {
    let mut i = 0;

    while i < len {
        *destination.add(i) = *source.add(i);

        i += 1;
    }

    destination
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memset(destination: *mut u8, value: i32, len: usize) -> *mut u8 {
    let mut i = 0;

    while i < len {
        *destination.add(i) = value as u8;

        i += 1;
    }

    destination
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn memcmp(this: *const u8, other: *const u8, len: usize) -> i32 {
    let mut i = 0;

    while i < len {
        let a = *this.add(i);
        let b = *other.add(i);

        if a != b {
            return a as i32 - b as i32;
        }

        i += 1;
    }

    0
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn strlen(this: *const u8) -> usize {
    let mut end = this;

    while *end != 0 {
        end = end.add(1);
    }

    end.sub_ptr(this)
}
