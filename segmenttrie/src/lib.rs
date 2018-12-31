#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::marker::PhantomData;

use fasthash::city::hash32;
use std::slice;
use std::iter::{FromIterator, Iterator};

pub fn into_bytes<'s, T: Sized>(obj: &'s T) -> &'s [u8] {
    let newlen = std::mem::size_of::<T>();
    let ptr : *const T = obj;
    unsafe { slice::from_raw_parts(ptr as *const u8, newlen) }
}

pub fn hash_obj<'s, T: Sized>(obj: &'s T) -> u32 {
    hash32(&into_bytes(obj))
}

pub fn hash_str(obj: &str) -> i32 {
    hash32(&obj) as i32
}

pub fn segment(path: &str) -> SimdVec<i32> {
    let v = path.split("/").map(hash_str).collect::<Vec<i32>>();
    v.into_iter().collect()
}

pub fn debug_mm256(mm: &__m256i) {
    let v : ResultVec = Default::default();
    unsafe { _mm256_store_si256(&v.v as *const i32 as *mut __m256i, *mm) }

    println!("mm256 as i32 : {:?}", v.v);
}

#[derive(Clone, Default, Debug)]
pub struct SimdVec<T> {
    v: Vec<__m256i>,
    len: usize,
    _p: PhantomData<T>,
}


#[repr(align(32))]
#[derive(Clone, Default, Debug)]
struct ResultVec {
    v: [i32; 8],
}

impl SimdVec<i32> {

    /// return the number of contiguous, equal cells between this vec and the other
    pub fn num_prefix_matches(&self, other: &SimdVec<i32>) -> i32 {
     
        let count_arr : ResultVec = Default::default();

        let mut count = 0;
        let mut o = other.v.iter();
        for i in self.v.iter() {
            if let Some(o1) = o.next() {
                let res = unsafe { _mm256_cmpeq_epi32(*i, *o1) };
                unsafe { _mm256_store_si256(&count_arr.v as *const i32 as *mut __m256i, res) }
                for i in count_arr.v.iter() {
                    if *i == 0 {
                        return count;
                    }
                    count += 1
                }
            } else {
                break
            }
        }

        count
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl FromIterator<i32> for SimdVec<i32> {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> SimdVec<i32> {
        let mut v : SimdVec<i32> = SimdVec { v: Vec::new(), len: 0, _p: PhantomData };
        let mut it = iter.into_iter();
        let mut arr: [i32; 8] = [0;8];
        loop { 
            if let Some(ref x) = it.next() {
                arr[0] = *x;
                v.len += 1;
            } else {
                break;
            }

            let mut i = 1;
            for x in it.by_ref() {
                arr[i] = x;
                v.len += 1;
                i += 1;
                if i == 8 {
                    break;
                }
            }

            let mm = unsafe { _mm256_set_epi32(
                        arr[7],
                        arr[6],
                        arr[5],
                        arr[4],
                        arr[3],
                        arr[2],
                        arr[1],
                        arr[0])};
            debug_mm256(&mm);
            v.v.push(mm);

            if arr[7] == 0 {
                break;
            }
        }

        v
    }
}

#[test]
fn it_works() {
    let v1 = segment("this/is/a/test/of/the/emergency/broadcast");
    let v2 = segment("this/is/a/test/do/not/panic");
    let num = v1.num_prefix_matches(&v2);
    assert_eq!(num, 4);
    assert_eq!(v1.len(), 8);
    assert_eq!(v2.len(), 7);
    let v1 = segment("this/is/a/test/of/the/emergency/broadcast/system/it/is/a/really/long/string/with/tons/of/stuff");
    let v2 = segment("this/is/a/test/of/the/emergency/broadcast/system/it/is/not/as/long/as/the/other");
    let num = v1.num_prefix_matches(&v2);
    assert_eq!(num, 11);
    assert_eq!(v1.len(), 19);
    assert_eq!(v2.len(), 17);
}
