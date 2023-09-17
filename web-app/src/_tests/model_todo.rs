use crate::model::db::init_db;
use crate::model::todo::TodoMac;

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    // -- AXTION
    let todos = TodoMac::list(&db).await?;

    // -- CHECL
    // assert_eq!(2, todos.len());
    println!("\n\n->> {:?}", todos);

    Ok(())
}
