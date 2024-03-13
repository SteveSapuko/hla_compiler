use crate::definitions::*;

#[derive(Debug, Clone)]
pub struct VarData {
    pub name: String,
    pub var_type: VarType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Pointer(Box<VarType>),
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    UserStruct(UserStructDef),
    UserEnum(UserEnumDef),
    Void,
    Array(u16, Box<VarType>)
}

impl VarType {
    pub fn size(&self) -> u16 {
        match self {
            Self::U8 => 1,
            Self::I8 => 1,
            Self::U16 => 2,
            Self::I16 => 2,
            Self::U32 => 4,
            Self::I32 => 4,
            Self::U64 => 8,
            Self::I64 => 8,
            Self::Pointer(_) => 2,
            Self::UserStruct(s) => {
                let mut sum: u16 = 0;
                for t in &s.fields {
                    sum += t.1.unwrap().size();
                }
                sum
            }
            Self::Void => 0,
            Self::Array(s, _) => *s,
            Self::UserEnum(_) => 1,
            Self::UserStruct(s) => {
                let mut sum: u16 = 0;
                for field in &s.fields {
                    sum += field.1.unwrap().size();
                }

                sum
            }
            _ => panic!()

        }
    }

    pub fn from(t: &str, defined_types: &Vec<UserType>) -> Result<Self, &'static str> {
        //println!("doing {}", t);
        match t.to_lowercase().as_str() {
            "u8" => Ok(Self::U8),
            "i8" => Ok(Self::I8),

            "u16" => Ok(Self::U16),
            "i16" => Ok(Self::I16),

            "u32" => Ok(Self::U32),
            "i32" => Ok(Self::I32),

            "u64" => Ok(Self::U64),
            "i64" => Ok(Self::I64),

            _ => {
                if t.len() >= 4 {
                    if &t[0..4] == "ptr@" {
                        return Ok(VarType::Pointer(Box::new(VarType::from(&t[4..], defined_types)?)))
                    }
                }

                if t == "void" {
                    return Ok(VarType::Void)
                }
                
                for user_type in defined_types.iter().rev() {
                    match user_type {
                        UserType::UserStruct(s)  => {
                            if s.name == t {
                                return Ok(VarType::UserStruct(s.clone()))
                            }
                        }

                        UserType::UserEnum(e) => {
                            if e.name == t {
                                return Ok(VarType::UserEnum(e.clone()))
                            }
                        }
                        
                    }
                }

                return Err("Undefined Type")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserType {
    UserStruct(UserStructDef),
    UserEnum(UserEnumDef),
}

impl UserType {
    pub fn name(&self) -> String {
        match self {
            UserType::UserStruct(s) => s.name.clone(),
            UserType::UserEnum(e) => e.name.clone() 
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct UserStructDef {
    pub name: String,
    pub fields: Vec<(String, FieldType)>
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Defined(VarType),
    Undefined(Token),
}

impl FieldType {
    pub fn unwrap(&self) -> VarType {
        match self {
            FieldType::Defined(t) => t.clone(),
            FieldType::Undefined(s) => panic!("Undefined Field Type {}", s.data())
        }
    }
}

impl UserStructDef {
    pub fn get_field_type(&self, name: String) -> Option<FieldType> {
        for f in self.fields.clone() {
            if f.0 == name {
                return Some(f.1)
            }
        }
        return None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserEnumDef {
    pub name: String,
    pub variants: Vec<String>
}


impl UserEnumDef {
    pub fn check_variant(&self, v_target: String) -> bool {
        for v in self.variants.clone() {
            if v == v_target {
                return true
            }
        }
        false
    }
}
