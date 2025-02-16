use wiki_indexers::card_indexer::indexer::CardIndexer;

#[tokio::test]
async fn indexes_character() -> Result<(), Box<dyn std::error::Error>> {
    let indexer = CardIndexer::new("http://localhost:3000");

    let result = indexer.index_char("2012").await?;

    assert_eq!(result.materials, vec![]);
    println!("{:#?}", result.character);
    for c in result.cards {
        println!("{:#?}", c);
    }
    for a in result.aliases {
        println!("{:#?}", a);
    }
    Ok(())
}
