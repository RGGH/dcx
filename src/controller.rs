use docx_rs::*;
use std::fs::*;
use std::io::Read;

use docx_rs::{Docx, Header, Paragraph, Pic, Run};

pub fn hello(paratxt : &str, headline:&str) -> Result<(), DocxError> {
    let path = std::path::Path::new("./hello2.docx");
    let file = std::fs::File::create(path).unwrap();

    // Make sure you have the correct directory and image
    let banner_pic = Pic::new(include_bytes!("./images/banner.jpg"));

    // Demo of a Style - this will be Arial - Centred
    let style1 = Style::new("Heading1", StyleType::Paragraph)
        .name("Heading1")
        .align(AlignmentType::Center)
         .fonts(RunFonts::new().ascii("Arial"));

    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Left Stuff"))),
        TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("RHS"))),
    ])]);

    let header = Header::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_image(banner_pic.clone())));
    let out = Vec::new();

    let footer =
        Footer::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Check out my Python360 channel on YouTube for more Docx creation code")));


    // Add An Image !
    let mut img = File::open("./images/cat.jpg").unwrap();
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf).unwrap();

    let pic = Pic::new(&buf).size(320 * 9525, 240 * 9525);

    let p1 = Paragraph::new()
        .add_run(Run::new().add_text(" Hello World")).size(36)
        .style("Heading1");

    Docx::new()
        .add_style(style1)
        .header(header)
        .footer(footer)
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(headline).size(36)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Adios")))
        .add_table(table)
        .add_paragraph(p1)
        .add_paragraph(
            Paragraph::new().add_hyperlink(
                Hyperlink::new("anchor", HyperlinkType::Anchor)
                    .add_run(Run::new().add_text("Hello this is an anchor!")),
            ),
        )
        .add_bookmark_start(1, "anchor")
        .add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text("How to use docx-rs")
                    .size(24)
                    .style("Heading1")
                    .fonts(RunFonts::new().ascii("Arial")),
            ),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_image(pic)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Chapter One").size(36)))
         .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Please consider subscribing if you find this useful and would like to help the channel - Thanks for watching!")))
         .add_paragraph(Paragraph::new().add_run(Run::new().add_text(paratxt)))
        .add_paragraph(
            Paragraph::new()
                .add_hyperlink(
                    Hyperlink::new("https://github.com/bokuweb/docx-rs/tree/main/docx-core", HyperlinkType::External).add_run(
                        Run::new().add_text(" This is the Hyperlink to the docx-rs GitHub"),
                    ),
                )
                .page_break_before(true),
        )
        .add_bookmark_end(1)
        .build()
        .pack(file)?;
    let _ = std::fs::write("./out.docx", out);

    Ok(())
}
