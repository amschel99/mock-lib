struct Credentials{
    name:String,
    password:String
}
enum Db_state{
    connected,
    interrupted
}
fn authenticate(creds:Credentials){
if let Db_state::connected= connect_to_db(){
    login(creds);
}
}

fn connect_to_db()->Db_state{
return Db_state::connected;
}



fn login (creds:Credentials){
    //do some authentication
    get_user();

}


fn get_user(){
//logout user

}

fn logout(){

    //logout user

    
}
