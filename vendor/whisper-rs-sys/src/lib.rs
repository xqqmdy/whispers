#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// rustc 1.99 warns on bindgen's bitfield transmutes; harmless generated code.
#![allow(unnecessary_transmutes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
