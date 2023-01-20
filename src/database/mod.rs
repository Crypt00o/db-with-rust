
use sqlx::{Error,MySql,Pool,query};


    pub struct DBClient{
       pub pool:Pool<MySql> 
    }
    
    impl DBClient{

         pub async fn new(mysql_host:&str , mysql_port:u16 , mysql_user:&str , mysql_pass:&str , mysql_database:&str )->Self{
        
            let  db_url:String=format!("mysql://{}:{}@{}:{}/{}",mysql_user,mysql_pass,mysql_host,mysql_port,mysql_database);
            match Pool::<MySql>::connect(db_url.as_str()).await
            {
                Ok(pool)=>return Self{pool},
                Err(err)=>{panic!("Error! : {}",err)}
            }
    
         }

        pub fn clone(&self)->Self{
            return Self{
                pool:self.pool.clone()
            };
        }

        pub fn drop(self)->(){
            self.close();
            return;
        }

        pub fn to_owned(self)->Self{
            return self;
        }


        pub async fn test_connection(&self)->Result<(),Error>{
 

                match  query("SELECT NOW(); ").fetch_one(&self.pool).await {
                    Ok(_row)=>println!("[+] Connected to Database Successfully "),
                    Err(err)=>return Err(err)
                }

         
            
                 return Ok(());
        
        }

        pub fn close(&self)->(){
            self.close();
            return;
        }

}
