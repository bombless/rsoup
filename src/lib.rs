use ::nodes::Document;
use ::parser::Parser;
use ::safety::Cleaner;
use ::safety::Whitelist;
use ::helper::DataUtil;

mod nodes;
mod parser;
mod safety;
mod helper;

pub struct Rsoup;

impl Rsoup {
    fn parse(html: String, baseUri: String, parser: Option<Parser>) -> Document {
        if let Some(parser) = parser {
            parser.parseInput(html, baseUri)
        } else {
            Parser::parse(html, baseUri)
        }
    }

    fn clean(bodyHtml: String, baseUri: String, whitelist: Whitelist, outputSettings: Option<&Document::OutputSettings>) -> String {
        let dirty : Document = parseBodyFragment(bodyHtml, baseUri);
        let cleaner : Cleaner = Cleaner::new(whitelist);
        if let Some(outputSettings) = outputSettings {
            clean.outputSettings(outputSettings);
        }
        clean.body().html()
    }

    fn isValid(bodyHtml: String, whitelist: Whitelist) -> bool {
        Cleaner::new(whitelist).isValidBodyHtml(bodyHtml)
    }
}