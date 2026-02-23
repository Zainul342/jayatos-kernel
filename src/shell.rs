use alloc::string::String;
use alloc::vec::Vec;
use crate::println;
use crate::print;
use crate::vga_buffer;

// Buffer global untuk menampung input user
static mut COMMAND_BUFFER: Option<String> = None;

pub fn init() {
    unsafe {
        COMMAND_BUFFER = Some(String::with_capacity(80));
    }
}

pub fn interpret_char(c: char) {
    match c {
        '
' => {
            // User menekan Enter, eksekusi perintah!
            println!("");
            execute_command();
            print_prompt();
        }
        '\u{0008}' => {
            // Handle Backspace (Kita butuh implementasi di VGA Buffer untuk hapus karakter)
            // Untuk sekarang, kita abaikan dulu atau anggap tidak ada backspace.
        }
        _ => {
            // Tambahkan karakter ke buffer dan cetak ke layar (Echo)
            unsafe {
                if let Some(ref mut buf) = COMMAND_BUFFER {
                    buf.push(c);
                    print!("{}", c);
                }
            }
        }
    }
}

fn execute_command() {
    let command = unsafe {
        COMMAND_BUFFER.as_mut().unwrap()
    };

    // Trim spasi
    let cmd = command.trim();

    if cmd.is_empty() {
        return;
    }

    match cmd {
        "help" => {
            println!("Available commands:");
            println!("  help  - Show this message");
            println!("  about - About ProgrammerOS Jayatos");
            println!("  clear - Clear the screen");
            println!("  echo  - Repeat your words");
        }
        "about" => {
            println!("ProgrammerOS Jayatos v0.1.0");
            println!("Built with Rust, zero bloat, pure performance.");
            println!("Target: Defeat Windows/macOS legacy systems.");
        }
        "clear" => {
            vga_buffer::WRITER.lock().clear_screen();
        }
        _ if cmd.starts_with("echo ") => {
            println!("{}", &cmd[5..]);
        }
        _ => {
            println!("Jayatos: command not found: '{}'", cmd);
        }
    }

    // Reset buffer setelah perintah selesai
    command.clear();
}

pub fn print_prompt() {
    print!("jayatos> ");
}
