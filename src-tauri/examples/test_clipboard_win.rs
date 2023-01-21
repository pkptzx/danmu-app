use clipboard_win::{formats, get_clipboard, set_clipboard};
use anyhow::{Result, Ok};
fn main()->Result<()> {
    

    let text = "my sample ><";
    
    set_clipboard(formats::Unicode, text).expect("To set clipboard");
 

    //Type is necessary as string can be stored in various storages
    let result: String = get_clipboard(formats::Unicode).expect("To set clipboard");
    assert_eq!(result, text);

    Ok(())
}