use serde::{Serialize,Deserialize};
use mapper::digest::Header;
use base64::encode;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq,Hash)]
pub enum Authorization{
    Authorization(Auth),
    JWT(String),
    APIKey(String),
    None,
}
impl Authorization{
    pub fn from_parts(tp:&str,value:String)->Self{
        match tp{
            "Basic"=>{
                let vals:Vec<&str> = value.split(":").collect();
                Self::Authorization(Auth::Basic(vals[0].to_string(),vals[1].to_string()))
            },
            "Bearer"=>Self::Authorization(Auth::Bearer(value)),
            "JWT"=>Self::JWT(value),
            "API-Key"=>Self::APIKey(value),
            _=>Self::None,
            
        }
    }
    pub fn get_header(&self)->Header{
        match self{
           Self::Authorization(Auth::Basic(username,password))=>{
                Header{
                    name:String::from("Authorization"),
                    value:format!("Basic {}",encode(format!("{}:{}",username,password))),
                }
           },
           Self::Authorization(Auth::Bearer(token))=>{
                Header{
                    name:String::from("Authorization"),
                    value:format!("Bearer {}",token),
                }
           },
           Self::JWT(token)=>{
                Header{
                    name:String::from("jwt"),
                    value:token.to_string(),
                }
           },
           Self::APIKey(key)=>{
                Header{
                    name:String::from("X-API-Key"),
                    value:key.to_string(),
                }
           },
           _=>{
               Header{
                   name:String::new(),
                   value:String::new(),
               }
           },
        }
    }
    pub fn is_api_key(&self)->bool{
        match self{
           Self::APIKey(_)=>true,
           _=>false,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq,Hash)]
pub enum Auth{
    Basic(String,String),
    Bearer(String),
    /*
    APIKey(String),
    Digest(String,String,String),
    OAuth2(String),
    Hawk(String,String,String),
    AWS(String,String),*/
    Other,
}
impl Default for Auth{
    fn default() -> Self {
        Self::Other
    }
}
/*
impl Auth{
    pub fn execute(&self){
        match self{
           Self::Basic(username,password)=>(),
           Self::Bearer(token)=>(),
           Self::APIKey(key)=>(),
           Self::Digest(username,password,realm)=>(), 
           Self::OAuth2(access_token)=>(), 
           Self::Hawk(id,key,algorithm)=>(), 
           Self::AWS(access_key,secret_key)=>(), 
           Self::Other=>(), 
        }
    }
}*/
