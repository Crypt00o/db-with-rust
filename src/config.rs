
pub mod config_env{

    use dotenv::dotenv as dotenv;
    use std::env::{vars,Vars};
    use std::collections::HashMap;

   pub fn dotenv_config()->Vars{
        dotenv().ok();
        return vars();
    } 

   pub fn dotenv_config_map()->HashMap<String,String>{

       let mut  env_map:HashMap<String,String>=HashMap::<String,String>::new();
       
       for (key,value) in dotenv_config(){
       
           if env_map.contains_key(&key) {
                match env_map.get_mut(&key)   {
                    Some(key)=>*key=value,
                    None=>{}
                } 
           }
           else{
                env_map.insert(key, value);
           }
            
        }

       return env_map;

   }
    

}
