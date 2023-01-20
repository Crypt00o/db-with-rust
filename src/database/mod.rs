
use sqlx::{Error,MySql,Pool,query, Row};
use sqlx::types::{chrono::{Utc,DateTime}};

    pub struct DBClient{
       pub pool:Pool<MySql>,
           db_url:String
    }
    
    impl DBClient{

         pub async fn new(mysql_host:&str , mysql_port:u16 , mysql_user:&str , mysql_pass:&str , mysql_database:&str )->Self{
        
            let  db_url:String=format!("mysql://{}:{}@{}:{}/{}",mysql_user,mysql_pass,mysql_host,mysql_port,mysql_database);
            match Pool::<MySql>::connect(db_url.as_str()).await
            {
                Ok(pool)=>return Self{pool,db_url},
                Err(err)=>{panic!("Error! : {}",err)}
            }
    
         }

        pub fn clone(&self)->Self{
            return Self{
                db_url: self.db_url.clone(),
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


        pub async fn test_connection(&self)->Result<String,Error>{
 

                match  query("SELECT NOW(); ").fetch_one(&self.pool).await {
                
                    Ok(row)=>{ 
                        let connecting_time:String= row.get::<DateTime<Utc>,usize>(0).to_rfc2822();
                        return Ok(connecting_time);

                    },
                    Err(err)=>return Err(err)
                };

        
        
        }

        pub  fn close(&self)->(){
            self.pool.close();
            return;
        }


        pub async fn reconnect(&mut self)->(){
            
            if self.pool.is_closed(){
                
                match Pool::<MySql>::connect(&self.db_url.as_str()).await{
                     
                     Ok(pool)=>{
                         self.pool=pool;
                         return ;
                     }
                     ,
                     Err(err)=>panic!("Error! While Reconnecting DB : {}",err)
                };                
            }
            
            else {
                return ;
            }
            
        }

}
