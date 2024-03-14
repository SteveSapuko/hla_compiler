use super::DeclrType;

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
    Array(Box<VarType>, u16)
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
            Self::Array(t, s) => t.size() * s,
            Self::UserEnum(_) => 1,
            _ => panic!()

        }
    }

    pub fn from(t: DeclrType, defined_types: &Vec<UserType>) -> Result<Self, &'static str> {
        //println!("doing {}", t);

        match t {
            DeclrType::BasicType(o_token) => {
                let t = o_token.data();
                
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

            DeclrType::Array(a_type, a_size) => {
                let temp = VarType::from(*a_type.clone(), defined_types)?;
                let a_size_int = match a_size.ttype.data().parse::<u16>() {
                    Ok(t) => t,
                    Err(_) => return Err("Cannot Parse Array Size"),
                };
                
                return Ok(VarType::Array(Box::new(temp), a_size_int))
            }

            DeclrType::Pointer(points_to) => {
                let temp = VarType::from(*points_to.clone(), defined_types)?;
                return Ok(VarType::Pointer(Box::new(temp)))
            }
        }

        
    }

    pub fn to_string(&self) -> String {
        match self.clone() {
            Self::U8 => "u8".to_string(),
            Self::I8 => "i8".to_string(),
            Self::U16 => "u16".to_string(),
            Self::I16 => "i16".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I32 => "i32".to_string(),
            Self::U64 => "u64".to_string(),
            Self::I64 => "i64".to_string(),

            Self::Pointer(t) => t.to_string(),
            Self::Array(t, s) => {
                let mut temp = String::new();
                temp.push_str("[");
                temp.push_str(t.to_string().as_str());
                temp.push_str(";");
                temp.push_str(format!("{}",s).as_str());
                temp.push_str("]");
                temp
            }

            Self::UserEnum(e) => e.name,
            Self::UserStruct(s) => s.name,
            Self::Void => "void".to_string()

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
    Undefined(DeclrType),
}

impl FieldType {
    pub fn unwrap(&self) -> VarType {
        match self {
            FieldType::Defined(t) => t.clone(),
            FieldType::Undefined(s) => panic!("Undefined Field Type {}", s)
        }
    }

    pub fn name_to_string(&self) -> String {
        match self.clone() {
            Self::Defined(t) => t.to_string(),
            Self::Undefined(t) => {
                if let DeclrType::BasicType(basic) = t {
                    basic.ttype.data()
                } else {
                    //this might need to be changed
                    "".to_string()
                }
            }
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
