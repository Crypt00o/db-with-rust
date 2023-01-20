mod config;
mod database;
mod common_traits;



use config::config_env::dotenv_config_map;
use database::DBClient;
use std::collections::HashMap;


/* define a function to get env var value by  env map  " which returned from my defined function  for setting
 new map from env vars check  dotenv_config_map @config.rs " , and key  then return env value
*/

fn get_envvalue_by_envkey<'env>(some_map:&'env HashMap<String,String> ,key:& 'env str)->&'env str{

    match some_map.get(&key.to_string()) {
        Some(value)=> return  (*value).as_str(),
        None=>return ""
    };
}



#[tokio::main]
async fn main(){
   
    // assign env vars to local vars 
    
    let env_vars=dotenv_config_map();
    
    let mysql_host:&str= get_envvalue_by_envkey(&env_vars,"MYSQL_HOST" );
    let mysql_port:u16= get_envvalue_by_envkey(&env_vars,"MYSQL_PORT" ).parse().unwrap_or(3306);
    let mysql_user:&str= get_envvalue_by_envkey(&env_vars,"MYSQL_USER" );
    let mysql_pass:&str= get_envvalue_by_envkey(&env_vars,"MYSQL_PASS" );
    let mysql_database:&str= get_envvalue_by_envkey(&env_vars,"MYSQL_DB" );
    

    // generate a pool client to use it for connecting 
    
    let pool_client=DBClient::new(mysql_host, mysql_port, mysql_user, mysql_pass, mysql_database).await;
    
    // use pool client for connecting db and get time  
    
    match pool_client.test_connection().await {
        Ok(connection_result)=>println!("[+] Connection Established to DB Successfully  : {}",connection_result),
        Err(err)=>panic!("Error ! : {}",err)
    };

    // closeing pool Connection 

    pool_client.close();



  }
