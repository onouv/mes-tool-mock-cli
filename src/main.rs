mod tool;
use tool::*;

fn main() {
    let tool = Tool::new("1");
    // println!("{}", tool);

    let tool = tool.handle(Signal::Start);
    // println!("{}", tool);
    
    let tool = tool.handle(Signal::LoadPart(Part {}));
    // println!("{}", tool);
    let tool = tool.handle(Signal::LoadPart(Part {}));
    // println!("{}", tool);
    let tool = tool.handle(Signal::LoadPart(Part {}));
    // println!("{}", tool);
    let tool = tool.handle(Signal::UnloadPart);
    // println!("{}", tool);
}