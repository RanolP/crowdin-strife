use std::io::{stdin, stdout, BufRead, Write};

use engine::{
    db::{MinecraftPlatform, SearchTmQuery, SqlxDatabase, TmDatabase},
    language::Language,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let database = SqlxDatabase::connect().await?;

    let stdin = stdin();
    let mut stdin = stdin.lock();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    loop {
        write!(stdout, "검색 >> ")?;
        stdout.flush()?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        let text = buf.trim();

        let (inferred_source, inferred_target) =
            if Language::infer_from_text(text).contains(&Language::Korean) {
                (Language::Korean, Language::English)
            } else {
                (Language::English, Language::Korean)
            };

        let result = database
            .search(SearchTmQuery {
                source: inferred_source,
                target: inferred_target,
                platform: MinecraftPlatform::Java,
                text: text.to_string(),
                skip: 0,
                take: 10,
            })
            .await?;
        writeln!(stdout, "{}", result.game_version)?;

        for word in result.list.items {
            writeln!(
                stdout,
                "{} => {}",
                word.source.content,
                word.targets
                    .first()
                    .map(|target| target.content.as_ref())
                    .unwrap_or("\x1B[3m번역 없음\x1B[0m")
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
