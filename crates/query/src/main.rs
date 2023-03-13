use std::io::{stdin, stdout, BufRead, Write};

use engine::db::{
    MinecraftPlatform, PrismaDatabase, SearchTmQuery, SourceLanguage::Auto, TmDatabase,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let database = PrismaDatabase::connect().await?;

    let stdin = stdin();
    let mut stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    loop {
        write!(stdout, "검색 >> ")?;
        stdout.flush()?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        let result = database
            .search(SearchTmQuery {
                source: Auto,
                platform: MinecraftPlatform::Java,
                text: buf.trim().to_string(),
                skip: 0,
                take: 10,
            })
            .await?;
        writeln!(stdout, "{}", result.game_version)?;

        for word in result.list.items {
            writeln!(
                stdout,
                "{} => {}",
                word.source.content, word.targets[0].content
            )?;
        }
        writeln!(
            stdout,
            "1 / {} - 전체 {} 개",
            (result.list.total + 9) / 10,
            result.list.total
        )?;
        writeln!(stdout)?;
    }
}
