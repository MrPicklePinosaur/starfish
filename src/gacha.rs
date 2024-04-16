
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref GACHA_TABLE: HashMap<GachaRarity, Vec<GachaItem>> = HashMap::from([
        (GachaRarity::Common, vec![
            GachaItem {
                name: "Xiangling",
                description: "",
                element: GachaElement::Fire,
                art: XIANGLING,
            }
        ])
    ]);
}

#[derive(Hash, Eq, PartialEq)]
pub enum GachaRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

pub enum GachaElement {
    Fire,
    Ice,
    Wind,
    Rock,
    Lightning,
}

pub struct GachaItem {
    pub name: &'static str,
    pub description: &'static str,
    pub element: GachaElement,
    pub art: &'static str,
}

const DILUC: &'static str = r#"
           ▒▒▒▒▒▒▒▒▒▒▒▒         
       ░░░▒▒▒▒▒▒▒▒▒▒▒▒▒         
      ░ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒   
     ▒░▒▒▒▓▒▒▓▒▒▒▒▒▒▒▒▒▒▒░░░    
     ░▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▒▒▒     
      ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░    
     ▒░▒▒▒▒▒▓▓▒▒▒░█▓░▒▒▒▒▒░░    
     ░░▒▒▒▒▓▒▓▓░▒▓▒▒▓▒▒▒░░░░    
       ░░░▒█████████▓▒▒▒▒░      
        ▒░░░███████▓░▒▒▒▒░      
      ░░░░░░░▒████▒░░▒▒░░░░     
    ░░░░░░░░░▒▓▓▓▓▒░▒▒░░░░░░    
  ░░▒░░░░░░░▒░░░░░░░░░░░░░░▒░░  
 ░░░░░▒░░░░░░░░▒▒░░░░░░░░▒░░░░░ 
  ▒░░░░▒░░░▒░░░▒▒░▒░▒░▒▒▒░░░░░  
░░░▒▒░░░░░░▒░▒░░░░░░▒▒▓▒▓▒░▒░░░░
░▒░░▒░░░░░▒▓▒░░░░░░▒█▒▒▓░░░▒░░▒░
"#;

const XIANGLING: &'static str = r#"
        ░░          ░           
      ░▒░░▒▒▒▒▒▒▒▒▒▒░▒░▒▒       
      ░▒░░▒▒▒▒▒▒▒▒▒▒░▒▒░▒░      
      ░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░      
       ░▒▒▒▓▒▒▒▒▒▒▒▒▒▒▒▒░       
       ▒▒▒▒▒▓▓▒▒▒▒▒▓▒▒▒▒▒       
       ░▒▒░▒▓██████▓▓▒▒▒░       
       ░▒▒▒█▓▓████▓▓█▒▒▒░       
       ▒▒▒▒▓████████▓▒▒▒░       
       ░░▒░░▒██████▒▒▒░░░       
         ░▒▓▓▒███▓▒▓▓▒░         
          ░▒▓▒▒░░▒░▓▒░          
         ░░▓██░▓▒░██▓░░         
       ▓██░▓▓▓████▓▓▓░██▒       
      ▓███▓▒▓▓▓▓▓▓▓▓▒▓███▓      
     ▓████▒█▒▓▓▓▓▓▓▒▓▓████▓     
   ░▒▒▓██▓░░░▒▓▓▒▓░░░░▓████▓    
"#;

// pub fn gacha_get() -> GachaItem {

// }
