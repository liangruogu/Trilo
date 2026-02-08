use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WordEntry {
    pub word: String,
    pub phonetic: Option<String>,
    pub definition: Option<String>,
    pub pos: Option<String>,
    pub collins: Option<i32>,
}

fn lookup_word(word: String) -> Result<Option<WordEntry>, Box<dyn std::error::Error>>{
    let conn = Connection::open("../../resources/ecdict.db")?;
    
    // 测试一个专业词汇
    let target = &word;
    let mut stmt = conn.prepare("SELECT definition FROM stardict WHERE word = ?")?;
    
    let definition: String = stmt.query_row([target], |row| row.get(0))?;
    
    println!("【{}】的离线释义如下：\n{}", target, definition);
    Ok(None)
}

fn main() {
    println!("{:?}", lookup_word("Hello".to_string()));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lookupword() {
        assert!(lookup_word("Hello".to_string()), WordEntry {
            word: "你好"
        });
    }
}