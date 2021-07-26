#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem = 1000;
    let response = reqwest::get(format!("https://www.acmicpc.net/problem/{}", problem))
        .await?
        .text()
        .await?;

    let document = scraper::Html::parse_document(&response);

    fn select<'a>(html: &'a scraper::Html, selector: &str) -> Option<scraper::ElementRef<'a>> {
        html.select(&scraper::Selector::parse(selector).unwrap())
            .next()
    }

    fn select_inner_text(html: &scraper::Html, selector: &str) -> String {
        select(html, selector)
            .map(|e| e.text().collect::<Vec<_>>().join(" "))
            .unwrap_or("NONE".to_string())
    }

    let problem = select_inner_text(&document, "#problem_description > p");
    let input = select_inner_text(&document, "#problem_input > p");
    let output = select_inner_text(&document, "#problem_output > p");
    let limit = select_inner_text(&document, "#problem_limit > p");

    println!("문제: {}", problem);
    println!("입력: {}", input);
    println!("출력: {}", output);
    println!("제한: {}", limit);

    Ok(())
}
