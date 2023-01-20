pub trait Common{
    
   fn new()->Self;
   fn clone(&self)->Self;
   fn to_owned(self)->Self;
   fn drop(self)->();
        
}
