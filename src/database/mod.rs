

use sqlx::{Error,MySql,Pool,query};
use sqlx::pool::{PoolConnection};

    pub async fn client(mysql_host:&str , mysql_port:u16 , mysql_user:&str , mysql_pass:&str , mysql_database:&str )->Result <Pool<MySql> , Error>{
        
        let  db_url:String=format!("mysql://{}:{}@{}:{}/{}",mysql_user,mysql_pass,mysql_host,mysql_port,mysql_database);
        
        return  Pool::<MySql>::connect(db_url.as_str()).await
    }

 

    pub async fn test_connection(client:&Pool<MySql>)->Result<(),Error>{

        let mut connection :PoolConnection<MySql>= match client.acquire().await  {
            Ok(connection_result)=>{connection_result},
            Err(err)=>return  Err(err)
        };

        match  query("SELECT NOW(); ").fetch_one(&mut connection).await {
        Ok(_row)=>println!("[+] Connected to Database Successfully "),
        Err(err)=>return Err(err)
        }

         
            
        return Ok(());
        
    }
