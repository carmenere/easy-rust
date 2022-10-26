# Example 1
```Rust
#[warn(unused_variables)]
#[warn(non_snake_case)]
#[allow(unused_macros)]

macro_rules! empty_macros {
    () => {
        println!("Empty macros!")
    };
}

macro_rules! simple_macros {
    ($name: expr) => {
        println!("Hi, {}", $name)
    };
}

macro_rules! var_macros {
  (
    $($opt: expr),*   // comma ',' is out of parentheses, because it isn't part of captured value
  ) => {
        $(println!("{}", $opt);)*
  };
}

macro_rules! n_parameters_macros {
    (x => $xx: expr, y => $yy: expr) => {
        println!("X is {}, Y is {}.", $xx, $yy)
    };
    
    (z => $zz: expr) => {
        println!("Z is {}", $zz)
    };
}

macro_rules! gen_enum {
    ($enum_name: ident) => {
        #[derive(Debug)]
        enum $enum_name {
            a,
            b
        }
    }
}

fn main() {
    empty_macros!();
    simple_macros!("ABC");
    var_macros!("X", "Y", "Z");
    n_parameters_macros!(x => "a", y => "b");
    n_parameters_macros!(z => "z");
    gen_enum!(qwerty);
    println!("{:?}", qwerty::a);
}
```

<br>

# Example 2
```Rust
macro_rules! extend_struct {
    (struct $base:ident {
        $($field_name:ident: $field_type:ty,)*   // comma ',' is inside parentheses, because it's part of captured value
    },
    struct $extended:ident {
        $($field_name2:ident: $field_type2:ty,)*
    }) => {
        struct $base {
            $($field_name: $field_type,)*
        }

        struct $extended {
            $($field_name: $field_type,)*
            $($field_name2: $field_type2,)*
        }
    }
}

extend_struct! (
    struct VnfdWithoutId {
        a: String,
        b: String,
    },
    struct VnfdWithId {
        id: u64,
    }
);
```

<br>

# Example 3: import macros as any other item
### File: ``m.rs``
```Rust
macro_rules! empty_macros {
    () => {println!("Empty macros")}
}

pub(crate) use empty_macros;
```

<br>

### File: ``main.rs``
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

mod m;

use m::{empty_macros};

fn main() {
    empty_macros!();
}
```

<br>

# Example 4
```Rust
macro_rules! gen_enum {
  (
    enum $name:ident {
        $($variant:ident = $val:expr,)*   // comma ',' is inside parentheses, because it's part of captured value
    }
  ) => {
        #[derive(Serialize, Deserialize)]
        enum $name {
            $($variant = $val),*
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                match self {
                    $($name::$variant => write!(f, stringify!($variant))),*
                }
            }
        }

        impl $name {
            fn variant_to_int(&self) -> i32 {
                match self {
                    $($name::$variant => $val),*,
                    _ => panic!("Got unsupported variant!")
                }
            }
        }

        fn int_to_variant(value: i32) -> $name {
            match value {
                $($val => $name::$variant),*,
                _ => panic!("Got unsupported number!")
            }
        }  

    };
}

gen_enum! {
    enum ProtoNumbers {
        tcp = 6,
        udp = 17,
        icmp = 1,
        ipip = 94,
        any = 512,
    }
}
```

<br>

# Example 5
```Rust
macro_rules! gen_structs {
  (
    struct_name => $struct_name:ident,
    pub_ref_fields => {
        $(pub $field_name:ident: $field_type:ty,)*
    },
    pub_no_ref_fields => {
        $(pub $field_nr_name:ident: $field_nr_type:ty,)*
    },
    private_fields => {
        $($pfield_name:ident: $pfield_type:ty,)*
    },
    no_id_suf => $no_id_suf:ident,
    del_suf => $del_suf:ident,
    ref_suf => $ref_suf:ident,
    orm_suf => $orm_suf:ident,
    id_name => $id_name:ident, 
    id_type => $id_type:ty
  ) => {
        #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
        #[serde(deny_unknown_fields)]
        pub struct $struct_name {
            pub $id_name: $id_type,
            $(pub $field_name: $field_type,)*
            $(pub $field_nr_name: $field_nr_type,)*
            $($pfield_name:ident: $pfield_type:ty,)*
        }
        paste! {
            #[derive(Debug, Deserialize,Serialize)]
            #[serde(deny_unknown_fields)]
            pub struct [<$struct_name $no_id_suf>]  {
                $(pub $field_name: $field_type,)*
                $(pub $field_nr_name: $field_nr_type,)*
                $($pfield_name:ident: $pfield_type:ty,)*
            }
            #[derive(Debug, Serialize)]
            pub struct [<$struct_name $ref_suf>]<'a> {
                pub $id_name: $id_type,
                $(pub $field_name: &'a $field_type,)*
                $(pub $field_nr_name: $field_nr_type,)*
                $($pfield_name:ident: $pfield_type:ty,)*
            }

            #[derive(Debug, Serialize)]
            pub struct [<$struct_name $orm_suf>] {
                pub $id_name: $id_type,
                $(pub $field_name: $field_type,)*
                $(pub $field_nr_name: $field_nr_type,)*
                $($pfield_name:ident: $pfield_type:ty,)*
            }
            #[derive(Debug, Serialize, Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct [<$struct_name $del_suf>] {
                pub $id_name: $id_type,
            }
        }
    }
}


gen_structs! {
    struct_name => Vnfd,
    pub_ref_fields => {
        pub name: String,
        pub version: String,
        pub vnf_type: String,
        pub vendor_name: String,
        pub vendor_version: String,
        pub description: String,
        pub url: String,
    }, 
    pub_no_ref_fields => {
        pub cpus: i32,
        pub ram: i32,
        pub disk: i32,
    }, 
    private_fields => {},
    no_id_suf => NoId,
    del_suf => Del,
    ref_suf => Ref,
    orm_suf => Row,
    id_name => vid, 
    id_type => i64
}
```
